//! Explicit recipient addressing: `docs/SPEC-recipient-addressing.md`.
//!
//! `send_message` and `reply_message` keep their string recipients. A
//! recipient may also be an object with exactly one selector, `name` or
//! `display_name`. `whois` gains a lookup by display name.
//!
//! The tools themselves are unchanged. [`SelectorRecipients`] and
//! [`WhoisByDisplayName`] wrap their handlers: a selector is resolved once to
//! the agent's stable name before the wrapped tool runs, so the send path
//! sees the stable addresses it has always seen.

use std::time::Duration;

use fastmcp::ToolHandler;
use fastmcp::prelude::*;
use fastmcp_protocol::{Icon, ToolAnnotations};
use fastmcp_server::BoxFuture;
use mcp_agent_mail_db::AgentRow;
use serde_json::{Map, Value, json};

use crate::tool_util::{
    db_outcome_to_mcp_result, get_coalescer_bypass_read_db_pool, get_db_pool_for_request,
    legacy_tool_error, resolve_existing_project,
};

/// The recipient lists of `send_message` and `reply_message`, in the order
/// in which a repeated agent keeps its first occurrence.
const RECIPIENT_FIELDS: [&str; 3] = ["to", "cc", "bcc"];
/// Argument names a caller may use for the project of a send or a reply.
const PROJECT_ARGUMENTS: [&str; 4] = ["project_key", "project", "project_slug", "human_key"];
const NAME_SELECTOR: &str = "name";
const DISPLAY_NAME_SELECTOR: &str = "display_name";

const RECIPIENT_DESCRIPTION: &str = "\n\nRecipient selectors\n-------------------\nEach item of `to`, `cc` and `bcc` is a string or an object with exactly one selector:\n- `\"BlueLake\"`: a stable address, as before.\n- `{\"name\": \"BlueLake\"}`: the same stable address, stated explicitly.\n- `{\"display_name\": \"SPX Maintainer\"}`: the one active agent of the sender's project that carries this display name. The label is trimmed and compared without regard to case.\nA selector is never guessed from a string and never retried as the other kind. A display name that no active agent carries fails with RECIPIENT_DISPLAY_NAME_NOT_FOUND; one that several carry fails with RECIPIENT_DISPLAY_NAME_AMBIGUOUS and lists their stable names; one that only retired agents carry fails with AGENT_RETIRED. A failed selector stops the whole call before anything is written, and a display name never registers an agent. Mail to another project keeps using stable address strings.";

const WHOIS_DESCRIPTION: &str = "\n\nLookup by display name\n----------------------\nPass `display_name` instead of `agent_name` to ask which active agents of the project carry a display name. The label is trimmed and compared without regard to case. The result is `{\"display_name\": str, \"matches\": [ ...one profile per agent... ]}`; `matches` is empty when no active agent carries the label. `display_name` and `agent_name` cannot be combined.";

/// One recipient object of a send or a reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipientSelector {
    /// A stable address, with the meaning and syntax of a recipient string.
    Name(String),
    /// A display name, looked up among the active agents of the sender's project.
    DisplayName(String),
}

fn invalid_recipient_object(field: &str, object: &Map<String, Value>, detail: &str) -> McpError {
    legacy_tool_error(
        "INVALID_ARGUMENT",
        format!(
            "Invalid recipient object in '{field}': {detail}. A recipient object has exactly one \
             selector: {{\"name\": \"<agent name>\"}} or {{\"display_name\": \"<display name>\"}}."
        ),
        true,
        json!({
            "argument": field,
            "invalid_item": Value::Object(object.clone()).to_string(),
        }),
    )
}

/// Read one recipient object. It carries exactly one selector, a string, and
/// no other field.
pub fn parse_recipient_object(
    field: &str,
    object: &Map<String, Value>,
) -> McpResult<RecipientSelector> {
    let name = object.get(NAME_SELECTOR);
    let display_name = object.get(DISPLAY_NAME_SELECTOR);
    if let Some(other) = object
        .keys()
        .find(|key| *key != NAME_SELECTOR && *key != DISPLAY_NAME_SELECTOR)
    {
        return Err(invalid_recipient_object(
            field,
            object,
            &format!("unknown field '{other}'"),
        ));
    }
    match (name, display_name) {
        (Some(_), Some(_)) => Err(invalid_recipient_object(
            field,
            object,
            "it has both 'name' and 'display_name'",
        )),
        (None, None) => Err(invalid_recipient_object(
            field,
            object,
            "it has neither 'name' nor 'display_name'",
        )),
        (Some(Value::String(name)), None) => Ok(RecipientSelector::Name(name.clone())),
        (None, Some(Value::String(label))) => {
            if label.trim().is_empty() {
                return Err(invalid_recipient_object(
                    field,
                    object,
                    "'display_name' is empty",
                ));
            }
            Ok(RecipientSelector::DisplayName(label.clone()))
        }
        (Some(_), None) => Err(invalid_recipient_object(
            field,
            object,
            "'name' must be a string",
        )),
        (None, Some(_)) => Err(invalid_recipient_object(
            field,
            object,
            "'display_name' must be a string",
        )),
    }
}

/// Check the shape of every recipient object in one list, without resolving it.
pub(crate) fn validate_recipient_objects(field: &str, items: &[Value]) -> McpResult<()> {
    for item in items {
        if let Value::Object(object) = item {
            parse_recipient_object(field, object)?;
        }
    }
    Ok(())
}

/// The form in which two display names are compared: trimmed, and without
/// regard to the case of any letter. Inner whitespace and Unicode form are
/// kept, so they must match.
fn comparison_form(label: &str) -> String {
    label.trim().to_lowercase()
}

/// The agents of one project that carry a display name. Deregistered agents
/// are absent: their identities can never receive mail again.
#[derive(Debug, Default)]
pub struct DisplayNameHolders {
    pub active: Vec<AgentRow>,
    pub retired: Vec<AgentRow>,
}

fn holders_in_roster(roster: &[AgentRow], label: &str) -> DisplayNameHolders {
    let wanted = comparison_form(label);
    let mut holders = DisplayNameHolders::default();
    for agent in roster {
        let carries = agent
            .display_name
            .as_deref()
            .is_some_and(|stored| comparison_form(stored) == wanted);
        if !carries {
            continue;
        }
        if agent.retired_at.is_some() {
            holders.retired.push(agent.clone());
        } else {
            holders.active.push(agent.clone());
        }
    }
    // The roster is ordered by activity. Report holders by stable name so
    // that the same state always yields the same answer.
    holders.active.sort_by(|a, b| a.name.cmp(&b.name));
    holders.retired.sort_by(|a, b| a.name.cmp(&b.name));
    holders
}

fn display_name_not_found_error(label: &str, project_slug: &str) -> McpError {
    let label = label.trim();
    legacy_tool_error(
        "RECIPIENT_DISPLAY_NAME_NOT_FOUND",
        format!(
            "No active agent in project '{project_slug}' carries the display name '{label}'. \
             Use resource://agents/{project_slug} to list agents with their display names, or \
             address the recipient by its stable name."
        ),
        true,
        json!({ "display_name": label, "project": project_slug }),
    )
}

fn display_name_ambiguous_error(label: &str, project_slug: &str, holders: &[AgentRow]) -> McpError {
    let label = label.trim();
    let candidates: Vec<&str> = holders.iter().map(|agent| agent.name.as_str()).collect();
    legacy_tool_error(
        "RECIPIENT_DISPLAY_NAME_AMBIGUOUS",
        format!(
            "{} active agents in project '{project_slug}' carry the display name '{label}': {}. \
             Address the one you mean by its stable name.",
            candidates.len(),
            candidates.join(", ")
        ),
        true,
        json!({
            "display_name": label,
            "project": project_slug,
            "candidates": candidates,
        }),
    )
}

/// The stable identity one display name addresses, or the reason it addresses none.
fn resolve_in_roster(roster: &[AgentRow], label: &str, project_slug: &str) -> McpResult<AgentRow> {
    let mut holders = holders_in_roster(roster, label);
    match holders.active.len() {
        1 => Ok(holders.active.remove(0)),
        0 => holders.retired.first().map_or_else(
            || Err(display_name_not_found_error(label, project_slug)),
            |retired| Err(crate::messaging::agent_retired_error(retired)),
        ),
        _ => Err(display_name_ambiguous_error(
            label,
            project_slug,
            &holders.active,
        )),
    }
}

/// A project's registered identities, read without creating the project.
async fn project_roster(
    ctx: &McpContext,
    pool: &mcp_agent_mail_db::DbPool,
    project_key: &str,
) -> McpResult<(mcp_agent_mail_db::ProjectRow, Vec<AgentRow>)> {
    let project = resolve_existing_project(ctx, pool, project_key).await?;
    let roster = db_outcome_to_mcp_result(
        mcp_agent_mail_db::queries::list_agent_roster(ctx.cx(), pool, project.id.unwrap_or(0))
            .await,
    )?;
    Ok((project, roster))
}

/// The stable names that display names address in one project, in the order
/// given. Any label that addresses no single active agent fails the call.
///
/// The lookup reads through the pool the send itself writes through. A
/// process that sends without a server holds the mailbox through that pool,
/// and a second, query-only lane beside it makes the send find the mailbox busy.
pub async fn resolve_display_names(
    ctx: &McpContext,
    project_key: &str,
    labels: &[String],
) -> McpResult<Vec<String>> {
    if labels.is_empty() {
        return Ok(Vec::new());
    }
    if let Some(empty) = labels.iter().find(|label| label.trim().is_empty()) {
        return Err(legacy_tool_error(
            "INVALID_ARGUMENT",
            "A display name must not be empty.",
            true,
            json!({ "argument": DISPLAY_NAME_SELECTOR, "invalid_item": empty }),
        ));
    }
    let pool = get_db_pool_for_request(ctx)?;
    let (project, roster) = project_roster(ctx, &pool, project_key).await?;
    labels
        .iter()
        .map(|label| resolve_in_roster(&roster, label, &project.slug).map(|agent| agent.name))
        .collect()
}

/// The active agents of one project that carry a display name, by stable
/// name. A pure read: it takes the query-only lane `whois` reads through.
pub async fn active_display_name_holders(
    ctx: &McpContext,
    project_key: &str,
    label: &str,
) -> McpResult<Vec<AgentRow>> {
    let pool = get_coalescer_bypass_read_db_pool()?;
    let (_, roster) = project_roster(ctx, &pool, project_key).await?;
    Ok(holders_in_roster(&roster, label).active)
}

fn project_argument(arguments: &Map<String, Value>) -> Option<&str> {
    PROJECT_ARGUMENTS
        .iter()
        .find_map(|name| arguments.get(*name).and_then(Value::as_str))
}

/// Replace every recipient object in the arguments of a send or a reply by
/// the stable address it selects.
///
/// Every object is checked before any is resolved, and every display name is
/// resolved against one reading of the project's agents. On an error the
/// arguments are left as they were.
pub async fn resolve_recipient_selectors(ctx: &McpContext, arguments: &mut Value) -> McpResult<()> {
    let Some(args) = arguments.as_object() else {
        return Ok(());
    };
    let mut selectors: Vec<(&str, usize, RecipientSelector)> = Vec::new();
    for field in RECIPIENT_FIELDS {
        let Some(Value::Array(items)) = args.get(field) else {
            continue;
        };
        for (index, item) in items.iter().enumerate() {
            if let Value::Object(object) = item {
                selectors.push((field, index, parse_recipient_object(field, object)?));
            }
        }
    }
    if selectors.is_empty() {
        return Ok(());
    }

    let labels: Vec<String> = selectors
        .iter()
        .filter_map(|(_, _, selector)| match selector {
            RecipientSelector::DisplayName(label) => Some(label.clone()),
            RecipientSelector::Name(_) => None,
        })
        .collect();
    let mut resolved = if labels.is_empty() {
        Vec::new()
    } else {
        let Some(project_key) = project_argument(args) else {
            return Err(legacy_tool_error(
                "INVALID_ARGUMENT",
                "A display-name recipient needs the sender's project: pass project_key.",
                true,
                json!({ "argument": "project_key" }),
            ));
        };
        resolve_display_names(ctx, project_key, &labels).await?
    }
    .into_iter();

    let mut addresses: Vec<(&str, usize, String)> = Vec::with_capacity(selectors.len());
    for (field, index, selector) in selectors {
        let address = match selector {
            RecipientSelector::Name(name) => name,
            RecipientSelector::DisplayName(_) => resolved
                .next()
                .ok_or_else(|| McpError::internal_error("display name left unresolved"))?,
        };
        addresses.push((field, index, address));
    }
    let Some(args) = arguments.as_object_mut() else {
        return Ok(());
    };
    for (field, index, address) in addresses {
        if let Some(Value::Array(items)) = args.get_mut(field)
            && let Some(item) = items.get_mut(index)
        {
            *item = Value::String(address);
        }
    }
    Ok(())
}

/// The schema of one recipient: a stable address string or a selector object.
fn recipient_item_schema() -> Value {
    json!({
        "type": ["string", "object"],
        "properties": {
            NAME_SELECTOR: { "type": "string" },
            DISPLAY_NAME_SELECTOR: { "type": "string" },
        },
        "additionalProperties": false,
        "minProperties": 1,
        "maxProperties": 1,
    })
}

fn append_description(definition: &mut Tool, addition: &str) {
    if let Some(description) = definition.description.as_mut() {
        description.push_str(addition);
    }
}

/// `send_message` or `reply_message`, accepting recipient objects.
///
/// The definition advertises the object forms. A call resolves them and then
/// runs the wrapped tool with stable address strings.
pub struct SelectorRecipients<T>(pub T);

impl<T: ToolHandler> ToolHandler for SelectorRecipients<T> {
    fn definition(&self) -> Tool {
        let mut definition = self.0.definition();
        if let Some(properties) = definition
            .input_schema
            .get_mut("properties")
            .and_then(Value::as_object_mut)
        {
            for field in RECIPIENT_FIELDS {
                if let Some(list) = properties.get_mut(field).and_then(Value::as_object_mut) {
                    list.insert("items".to_string(), recipient_item_schema());
                }
            }
        }
        append_description(&mut definition, RECIPIENT_DESCRIPTION);
        definition
    }

    fn icon(&self) -> Option<&Icon> {
        self.0.icon()
    }

    fn version(&self) -> Option<&str> {
        self.0.version()
    }

    fn tags(&self) -> &[String] {
        self.0.tags()
    }

    fn annotations(&self) -> Option<&ToolAnnotations> {
        self.0.annotations()
    }

    fn output_schema(&self) -> Option<Value> {
        self.0.output_schema()
    }

    fn timeout(&self) -> Option<Duration> {
        self.0.timeout()
    }

    fn call(&self, ctx: &McpContext, arguments: Value) -> McpResult<Vec<Content>> {
        self.0.call(ctx, arguments)
    }

    fn call_async<'a>(
        &'a self,
        ctx: &'a McpContext,
        arguments: Value,
    ) -> BoxFuture<'a, McpOutcome<Vec<Content>>> {
        Box::pin(async move {
            let mut arguments = arguments;
            match resolve_recipient_selectors(ctx, &mut arguments).await {
                Ok(()) => self.0.call_async(ctx, arguments).await,
                Err(error) => fastmcp_core::Outcome::Err(error),
            }
        })
    }
}

fn invalid_whois_argument(detail: &str) -> McpError {
    legacy_tool_error(
        "INVALID_ARGUMENT",
        detail,
        true,
        json!({ "argument": DISPLAY_NAME_SELECTOR }),
    )
}

/// `whois`, accepting `display_name` in place of `agent_name`.
///
/// A lookup by display name returns one profile of the wrapped tool per
/// active agent that carries the label. A lookup by `agent_name` is the
/// wrapped tool's, unchanged.
pub struct WhoisByDisplayName<T>(pub T);

impl<T: ToolHandler> WhoisByDisplayName<T> {
    async fn profiles_by_display_name(
        &self,
        ctx: &McpContext,
        arguments: &Map<String, Value>,
        label: &str,
    ) -> McpResult<Vec<Content>> {
        if label.trim().is_empty() {
            return Err(invalid_whois_argument("display_name must not be empty."));
        }
        let Some(project_key) = project_argument(arguments) else {
            return Err(legacy_tool_error(
                "INVALID_ARGUMENT",
                "whois by display name needs project_key.",
                true,
                json!({ "argument": "project_key" }),
            ));
        };
        let holders = active_display_name_holders(ctx, project_key, label).await?;
        let mut matches = Vec::with_capacity(holders.len());
        for agent in holders {
            let mut profile_arguments = arguments.clone();
            profile_arguments.remove(DISPLAY_NAME_SELECTOR);
            profile_arguments.insert("agent_name".to_string(), Value::String(agent.name));
            let contents = match self
                .0
                .call_async(ctx, Value::Object(profile_arguments))
                .await
            {
                fastmcp_core::Outcome::Ok(contents) => contents,
                fastmcp_core::Outcome::Err(error) => return Err(error),
                fastmcp_core::Outcome::Cancelled(_) => return Err(McpError::request_cancelled()),
                fastmcp_core::Outcome::Panicked(_) => {
                    return Err(McpError::internal_error("whois panicked"));
                }
            };
            let Some(Content::Text { text }) = contents.first() else {
                return Err(McpError::internal_error("whois returned no profile text"));
            };
            let profile: Value = serde_json::from_str(text).map_err(|error| {
                McpError::internal_error(format!("whois returned an unreadable profile: {error}"))
            })?;
            matches.push(profile);
        }
        let response = json!({
            DISPLAY_NAME_SELECTOR: label.trim(),
            "matches": matches,
        });
        Ok(vec![Content::Text {
            text: response.to_string(),
        }])
    }
}

impl<T: ToolHandler> ToolHandler for WhoisByDisplayName<T> {
    fn definition(&self) -> Tool {
        let mut definition = self.0.definition();
        if let Some(schema) = definition.input_schema.as_object_mut() {
            if let Some(required) = schema.get_mut("required").and_then(Value::as_array_mut) {
                required.retain(|name| name.as_str() != Some("agent_name"));
            }
            if let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) {
                properties.insert(
                    DISPLAY_NAME_SELECTOR.to_string(),
                    json!({ "type": ["string", "null"] }),
                );
            }
        }
        append_description(&mut definition, WHOIS_DESCRIPTION);
        definition
    }

    fn icon(&self) -> Option<&Icon> {
        self.0.icon()
    }

    fn version(&self) -> Option<&str> {
        self.0.version()
    }

    fn tags(&self) -> &[String] {
        self.0.tags()
    }

    fn annotations(&self) -> Option<&ToolAnnotations> {
        self.0.annotations()
    }

    fn output_schema(&self) -> Option<Value> {
        self.0.output_schema()
    }

    fn timeout(&self) -> Option<Duration> {
        self.0.timeout()
    }

    fn call(&self, ctx: &McpContext, arguments: Value) -> McpResult<Vec<Content>> {
        self.0.call(ctx, arguments)
    }

    fn call_async<'a>(
        &'a self,
        ctx: &'a McpContext,
        arguments: Value,
    ) -> BoxFuture<'a, McpOutcome<Vec<Content>>> {
        Box::pin(async move {
            let label = match arguments.get(DISPLAY_NAME_SELECTOR) {
                None | Some(Value::Null) => return self.0.call_async(ctx, arguments).await,
                Some(Value::String(label)) => label.clone(),
                Some(_) => {
                    return fastmcp_core::Outcome::Err(invalid_whois_argument(
                        "display_name must be a string.",
                    ));
                }
            };
            if arguments
                .get("agent_name")
                .is_some_and(|name| !name.is_null())
            {
                return fastmcp_core::Outcome::Err(invalid_whois_argument(
                    "Pass either agent_name or display_name to whois, not both.",
                ));
            }
            let Some(object) = arguments.as_object() else {
                return self.0.call_async(ctx, arguments).await;
            };
            match self.profiles_by_display_name(ctx, object, &label).await {
                Ok(contents) => fastmcp_core::Outcome::Ok(contents),
                Err(error) => fastmcp_core::Outcome::Err(error),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(name: &str, display_name: Option<&str>, retired: bool) -> AgentRow {
        AgentRow {
            id: None,
            project_id: 1,
            name: name.to_string(),
            program: String::new(),
            model: String::new(),
            task_description: String::new(),
            inception_ts: 0,
            last_active_ts: 0,
            attachments_policy: "auto".to_string(),
            contact_policy: "auto".to_string(),
            reaper_exempt: 0,
            registration_token: None,
            retired_at: retired.then_some(1),
            display_name: display_name.map(str::to_string),
        }
    }

    fn error_type(error: &McpError) -> String {
        crate::tool_util::tool_error_code(error)
            .unwrap_or_default()
            .to_string()
    }

    fn object(value: Value) -> Map<String, Value> {
        value.as_object().cloned().expect("an object")
    }

    #[test]
    fn a_recipient_object_carries_exactly_one_selector() {
        assert_eq!(
            parse_recipient_object("to", &object(json!({"name": "BlueLake"}))).unwrap(),
            RecipientSelector::Name("BlueLake".to_string())
        );
        assert_eq!(
            parse_recipient_object("cc", &object(json!({"display_name": "SPX Maintainer"})))
                .unwrap(),
            RecipientSelector::DisplayName("SPX Maintainer".to_string())
        );
        for invalid in [
            json!({}),
            json!({"name": "BlueLake", "display_name": "SPX Maintainer"}),
            json!({"display_name": "SPX Maintainer", "project": "other"}),
            json!({"agent": "BlueLake"}),
            json!({"name": 7}),
            json!({"display_name": ["SPX Maintainer"]}),
            json!({"display_name": "   "}),
        ] {
            let error = parse_recipient_object("to", &object(invalid.clone()))
                .expect_err("an invalid recipient object");
            assert_eq!(error_type(&error), "INVALID_ARGUMENT", "{invalid}");
        }
    }

    #[test]
    fn comparison_ignores_case_of_all_letters_and_outer_whitespace_only() {
        let roster = [agent("BlueLake", Some("Müller  Team"), false)];
        for label in ["Müller  Team", "MÜLLER  TEAM", "  müller  team\t"] {
            let found = resolve_in_roster(&roster, label, "p").expect(label);
            assert_eq!(found.name, "BlueLake");
        }
        // One inner space where the stored label has two.
        let error = resolve_in_roster(&roster, "Müller Team", "p").expect_err("inner whitespace");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        // The same letters in decomposed Unicode form.
        let error =
            resolve_in_roster(&roster, "Mu\u{308}ller  Team", "p").expect_err("Unicode form");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
    }

    #[test]
    fn a_display_name_never_matches_a_stable_name() {
        let roster = [
            agent("BlueLake", Some("GreenCastle"), false),
            agent("GreenCastle", None, false),
        ];
        let found = resolve_in_roster(&roster, "GreenCastle", "p").unwrap();
        assert_eq!(found.name, "BlueLake");
        let error = resolve_in_roster(&roster, "BlueLake", "p").expect_err("a stable name");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
    }

    #[test]
    fn several_active_holders_are_ambiguous_and_named() {
        let roster = [
            agent("RedFox", Some("Reviewer"), false),
            agent("BlueLake", Some("reviewer"), false),
            agent("GreenCastle", Some("Reviewer"), true),
        ];
        let error = resolve_in_roster(&roster, "Reviewer", "p").expect_err("ambiguous");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_AMBIGUOUS");
        let data = error.data.expect("error data");
        assert_eq!(
            data["error"]["data"]["candidates"],
            json!(["BlueLake", "RedFox"])
        );
        assert_eq!(data["error"]["data"]["display_name"], json!("Reviewer"));
    }

    #[test]
    fn an_active_holder_wins_over_a_retired_one() {
        let roster = [
            agent("GreenCastle", Some("Reviewer"), true),
            agent("BlueLake", Some("Reviewer"), false),
        ];
        assert_eq!(
            resolve_in_roster(&roster, "Reviewer", "p").unwrap().name,
            "BlueLake"
        );
    }

    #[test]
    fn a_label_held_only_by_retired_agents_reports_retirement() {
        let roster = [agent("GreenCastle", Some("Reviewer"), true)];
        let error = resolve_in_roster(&roster, "Reviewer", "p").expect_err("retired");
        assert_eq!(error_type(&error), "AGENT_RETIRED");
    }

    #[test]
    fn the_recipient_schema_admits_a_string_or_one_selector() {
        let schema = recipient_item_schema();
        assert_eq!(schema["type"], json!(["string", "object"]));
        assert_eq!(schema["additionalProperties"], json!(false));
        assert_eq!(schema["minProperties"], json!(1));
        assert_eq!(schema["maxProperties"], json!(1));
        assert_eq!(
            schema["properties"],
            json!({
                "name": { "type": "string" },
                "display_name": { "type": "string" },
            })
        );
    }
}
