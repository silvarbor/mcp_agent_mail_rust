//! Explicit recipient addressing through the tool handlers the server
//! registers: `docs/SPEC-recipient-addressing.md`.
//!
//! Each test names the acceptance criteria of the specification it covers.

use asupersync::Cx;
use asupersync::runtime::RuntimeBuilder;
use fastmcp::ToolHandler;
use fastmcp::prelude::{Content, McpContext, McpError};
use mcp_agent_mail_core::{Config, config::with_process_env_overrides_for_test};
use mcp_agent_mail_tools::recipient_addressing::{SelectorRecipients, WhoisByDisplayName};
use mcp_agent_mail_tools::{
    ReplyMessage, SendMessage, Whois, deregister_agent, ensure_project, fetch_inbox,
    register_agent, retire_agent, tool_error_code,
};
use serde_json::{Value, json};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_LOCK: Mutex<()> = Mutex::new(());
static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_suffix() -> u64 {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    u64::try_from(micros)
        .unwrap_or(u64::MAX)
        .wrapping_add(TEST_COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Run one test against its own database and archive, with a fresh project.
fn run_with_project<F, Fut>(f: F)
where
    F: FnOnce(McpContext, String) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let _lock = TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let suffix = unique_suffix();
    let database_url = format!("sqlite:///tmp/recipient-addressing-{suffix}.sqlite3");
    let storage_root = format!("/tmp/recipient-addressing-storage-{suffix}");
    std::fs::create_dir_all(&storage_root).expect("create test storage root");
    let storage_root = std::fs::canonicalize(&storage_root)
        .expect("canonical test storage root")
        .to_string_lossy()
        .into_owned();
    let env = [
        ("DATABASE_URL", database_url.as_str()),
        ("STORAGE_ROOT", storage_root.as_str()),
        ("MESSAGING_AUTO_REGISTER_RECIPIENTS", "true"),
        ("MESSAGING_FAIL_CLOSED_SEND_PROFILE", "0"),
        ("TMUX_PANE", ""),
    ];
    with_process_env_overrides_for_test(&env, || {
        Config::reset_cached();
        let rt = RuntimeBuilder::current_thread()
            .build()
            .expect("build runtime");
        rt.block_on(async {
            let cx = Cx::current().expect("runtime installs the tool test context");
            let ctx = McpContext::new(cx, 1);
            let project = format!("/tmp/recipient-addressing-project-{suffix}");
            ensure_project(&ctx, project.clone(), None)
                .await
                .expect("ensure project");
            f(ctx, project).await;
        });
        Config::reset_cached();
    });
}

/// Register an agent that accepts mail from anyone, optionally with a display
/// name, and return its profile, which carries the registration token.
async fn register(ctx: &McpContext, project: &str, name: &str, label: Option<&str>) -> Value {
    let raw = register_agent(
        ctx,
        project.to_string(),
        "test-client".to_string(),
        "test-model".to_string(),
        Some(name.to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        label.map(str::to_string),
    )
    .await
    .expect("register agent");
    mcp_agent_mail_tools::contacts::set_contact_policy(
        ctx,
        project.to_string(),
        name.to_string(),
        "open".to_string(),
    )
    .await
    .expect("open contact policy");
    serde_json::from_str(&raw).expect("agent JSON")
}

fn token(profile: &Value) -> Option<String> {
    profile["registration_token"].as_str().map(str::to_string)
}

async fn call<T: ToolHandler>(
    tool: &T,
    ctx: &McpContext,
    arguments: Value,
) -> Result<Value, McpError> {
    match tool.call_async(ctx, arguments).await {
        asupersync::Outcome::Ok(contents) => {
            let Some(Content::Text { text }) = contents.first() else {
                panic!("the tool returned no text");
            };
            Ok(serde_json::from_str(text).expect("tool JSON"))
        }
        asupersync::Outcome::Err(error) => Err(error),
        other => panic!("the tool neither answered nor failed: {other:?}"),
    }
}

async fn send(ctx: &McpContext, arguments: Value) -> Result<Value, McpError> {
    call(&SelectorRecipients(SendMessage), ctx, arguments).await
}

async fn reply(ctx: &McpContext, arguments: Value) -> Result<Value, McpError> {
    call(&SelectorRecipients(ReplyMessage), ctx, arguments).await
}

async fn whois(ctx: &McpContext, arguments: Value) -> Result<Value, McpError> {
    call(&WhoisByDisplayName(Whois), ctx, arguments).await
}

fn send_arguments(project: &str, sender: &str, subject: &str, recipients: Value) -> Value {
    let mut arguments = json!({
        "project_key": project,
        "sender_name": sender,
        "subject": subject,
        "body_md": "body",
    });
    let fields = arguments.as_object_mut().expect("arguments");
    let Value::Object(lists) = recipients else {
        panic!("recipient lists are an object");
    };
    fields.extend(lists);
    arguments
}

/// No message has reached the agent's inbox.
async fn assert_inbox_empty(ctx: &McpContext, project: &str, agent: &str) {
    assert_eq!(
        inbox_subjects(ctx, project, agent).await,
        Vec::<String>::new(),
        "{agent} received nothing"
    );
}

fn error_type(error: &McpError) -> &str {
    tool_error_code(error).unwrap_or("<no error type>")
}

fn error_data(error: &McpError) -> Value {
    error
        .data
        .as_ref()
        .map_or(Value::Null, |data| data["error"]["data"].clone())
}

/// The subjects in an agent's inbox, oldest first.
async fn inbox_subjects(ctx: &McpContext, project: &str, agent: &str) -> Vec<String> {
    let raw = fetch_inbox(
        ctx,
        project.to_string(),
        agent.to_string(),
        None,
        None,
        Some(50),
        Some(false),
        None,
        None,
        None,
        Some(false),
    )
    .await
    .expect("fetch inbox");
    let messages: Value = serde_json::from_str(&raw).expect("inbox JSON");
    let mut subjects: Vec<(i64, String)> = messages
        .as_array()
        .expect("inbox list")
        .iter()
        .map(|message| {
            (
                message["id"].as_i64().unwrap_or(0),
                message["subject"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    subjects.sort();
    subjects.into_iter().map(|(_, subject)| subject).collect()
}

fn names(list: &Value) -> Vec<&str> {
    list.as_array()
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

/// Criterion 1: string recipients behave as before through the wrapped
/// handler, registration on the spot included. The `name` selector accepts
/// what a string accepts.
#[test]
fn strings_and_name_selectors_keep_stable_address_behavior() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", None).await;

        let sent = send(
            &ctx,
            send_arguments(&project, "BlueLake", "plain", json!({"to": ["RedStone"]})),
        )
        .await
        .expect("a string recipient delivers");
        assert_eq!(names(&sent["deliveries"][0]["payload"]["to"]), ["RedStone"]);

        send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "explicit",
                json!({"to": [{"name": "redstone"}]}),
            ),
        )
        .await
        .expect("a name selector delivers, without regard to case");
        assert_eq!(
            inbox_subjects(&ctx, &project, "RedStone").await,
            ["plain", "explicit"]
        );

        // An unknown plain name is registered on the spot, as a string and as
        // a name selector alike.
        for (subject, recipient) in [
            ("on the spot", json!("GreenCastle")),
            ("on the spot, explicit", json!({"name": "PurpleHill"})),
        ] {
            send(
                &ctx,
                send_arguments(&project, "BlueLake", subject, json!({"to": [recipient]})),
            )
            .await
            .expect("an unknown local name is registered on the spot");
        }
        // A placeholder registered on the spot first receives the notice of
        // its approved contact with the sender, then the message.
        assert_eq!(
            inbox_subjects(&ctx, &project, "GreenCastle").await,
            ["Contact approved: BlueLake -> GreenCastle", "on the spot"]
        );
        assert_eq!(
            inbox_subjects(&ctx, &project, "PurpleHill").await,
            [
                "Contact approved: BlueLake -> PurpleHill",
                "on the spot, explicit"
            ]
        );
    });
}

/// Criteria 2 and 12: a unique display name delivers in each of `to`, `cc`
/// and `bcc`, and a blind copy stays hidden from the other recipients.
#[test]
fn a_unique_display_name_delivers_in_to_cc_and_bcc() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("SPX Maintainer")).await;
        register(&ctx, &project, "GreenCastle", Some("Night Reviewer")).await;
        register(&ctx, &project, "PurpleHill", Some("Auditor")).await;

        let sent = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "by label",
                json!({
                    "to": [{"display_name": "SPX Maintainer"}],
                    "cc": [{"display_name": "Night Reviewer"}],
                    "bcc": [{"display_name": "Auditor"}],
                }),
            ),
        )
        .await
        .expect("display names deliver");
        let payload = &sent["deliveries"][0]["payload"];
        assert_eq!(names(&payload["to"]), ["RedStone"]);
        assert_eq!(names(&payload["cc"]), ["GreenCastle"]);

        for agent in ["RedStone", "GreenCastle", "PurpleHill"] {
            assert_eq!(
                inbox_subjects(&ctx, &project, agent).await,
                ["by label"],
                "{agent}"
            );
        }
        let raw = fetch_inbox(
            &ctx,
            project.clone(),
            "RedStone".to_string(),
            None,
            None,
            Some(5),
            Some(true),
            None,
            None,
            None,
            Some(false),
        )
        .await
        .expect("fetch inbox");
        assert!(
            !raw.contains("PurpleHill") && !raw.contains("Auditor"),
            "a blind copy is hidden from the other recipients: {raw}"
        );
    });
}

/// Criterion 3: duplicate labels among active agents are ambiguous and name
/// the candidates; an absent label is not found.
#[test]
fn ambiguous_and_absent_labels_fail_with_their_own_errors() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;
        register(&ctx, &project, "GreenCastle", Some("reviewer")).await;

        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "ambiguous",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect_err("two active agents carry the label");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_AMBIGUOUS");
        let data = error_data(&error);
        assert_eq!(data["display_name"], "Reviewer");
        assert_eq!(names(&data["candidates"]), ["GreenCastle", "RedStone"]);

        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "absent",
                json!({"to": [{"display_name": "Release Captain"}]}),
            ),
        )
        .await
        .expect_err("no agent carries the label");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        assert_eq!(error_data(&error)["display_name"], "Release Captain");

        for agent in ["RedStone", "GreenCastle"] {
            assert_inbox_empty(&ctx, &project, agent).await;
        }
    });
}

/// Criteria 4 and 5: each selector searches only its own namespace, a failed
/// selector is not retried in the other, and a display name registers nobody.
#[test]
fn a_selector_searches_only_its_own_namespace() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        // RedStone's display name is GreenCastle's stable name.
        register(&ctx, &project, "RedStone", Some("GreenCastle")).await;
        register(&ctx, &project, "GreenCastle", Some("Night Reviewer")).await;

        send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "to the label",
                json!({"to": [{"display_name": "GreenCastle"}]}),
            ),
        )
        .await
        .expect("the display name selects its holder");
        send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "to the name",
                json!({"to": [{"name": "GreenCastle"}]}),
            ),
        )
        .await
        .expect("the stable name selects its owner");
        assert_eq!(
            inbox_subjects(&ctx, &project, "RedStone").await,
            ["to the label"]
        );
        assert_eq!(
            inbox_subjects(&ctx, &project, "GreenCastle").await,
            ["to the name"]
        );

        // A stable name is not a display name.
        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "name as label",
                json!({"to": [{"display_name": "RedStone"}]}),
            ),
        )
        .await
        .expect_err("no agent carries the display name RedStone");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");

        // A display name is not a stable name: neither as a selector nor as a string.
        for recipient in [json!({"name": "Night Reviewer"}), json!("Night Reviewer")] {
            let error = send(
                &ctx,
                send_arguments(
                    &project,
                    "BlueLake",
                    "label as name",
                    json!({"to": [recipient]}),
                ),
            )
            .await
            .expect_err("a display name is no stable address");
            assert_ne!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        }

        // An unused label that would be a valid new agent name registers nobody.
        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "no registration",
                json!({"to": [{"display_name": "PurpleHill"}]}),
            ),
        )
        .await
        .expect_err("no agent carries the display name PurpleHill");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        let error = whois(
            &ctx,
            json!({"project_key": project, "agent_name": "PurpleHill"}),
        )
        .await
        .expect_err("the display-name selector registered no agent");
        assert_eq!(error_type(&error), "AGENT_NOT_FOUND");

        assert_eq!(
            inbox_subjects(&ctx, &project, "RedStone").await,
            ["to the label"]
        );
        assert_eq!(
            inbox_subjects(&ctx, &project, "GreenCastle").await,
            ["to the name"]
        );
    });
}

/// Criterion 6: an object with both selectors, with neither, or with another
/// field is rejected, in every recipient list.
#[test]
fn malformed_recipient_objects_are_rejected() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;

        let malformed = [
            json!({"name": "RedStone", "display_name": "Reviewer"}),
            json!({}),
            json!({"agent": "RedStone"}),
            json!({"display_name": "Reviewer", "project": project}),
            json!({"name": "RedStone", "kind": "to"}),
            json!({"display_name": 7}),
            json!({"display_name": "  "}),
        ];
        for field in ["to", "cc", "bcc"] {
            for object in &malformed {
                let mut recipients = json!({"to": ["RedStone"]});
                recipients[field] = json!([object]);
                let error = send(
                    &ctx,
                    send_arguments(&project, "BlueLake", "malformed", recipients),
                )
                .await
                .expect_err("a malformed recipient object");
                assert_eq!(error_type(&error), "INVALID_ARGUMENT", "{field}: {object}");
                assert_eq!(error_data(&error)["argument"], field, "{object}");
            }
        }
        assert_inbox_empty(&ctx, &project, "RedStone").await;
    });
}

/// Criterion 7: case and surrounding whitespace are ignored, for all letters;
/// inner whitespace is not.
#[test]
fn comparison_ignores_case_and_surrounding_whitespace_only() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Müller  Team")).await;

        for (subject, label) in [
            ("exact", "Müller  Team"),
            ("upper", "MÜLLER  TEAM"),
            ("padded", "  müller  team \t"),
        ] {
            send(
                &ctx,
                send_arguments(
                    &project,
                    "BlueLake",
                    subject,
                    json!({"to": [{"display_name": label}]}),
                ),
            )
            .await
            .unwrap_or_else(|error| panic!("{label:?} resolves: {error:?}"));
        }
        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "inner",
                json!({"to": [{"display_name": "Müller Team"}]}),
            ),
        )
        .await
        .expect_err("inner whitespace differs");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        assert_eq!(
            inbox_subjects(&ctx, &project, "RedStone").await,
            ["exact", "upper", "padded"]
        );
    });
}

/// Criterion 8: an active holder wins over a retired one; a label held only
/// by retired agents reports the retirement; a label held only by
/// deregistered agents is not found.
#[test]
fn retired_and_deregistered_holders_are_not_candidates() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;
        let retiring = register(&ctx, &project, "GreenCastle", Some("Reviewer")).await;
        let leaving = register(&ctx, &project, "PurpleHill", Some("Auditor")).await;

        retire_agent(
            &ctx,
            project.clone(),
            "GreenCastle".to_string(),
            token(&retiring),
            None,
            None,
            None,
        )
        .await
        .expect("retire GreenCastle");
        send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "active wins",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect("the active holder receives");
        assert_eq!(
            inbox_subjects(&ctx, &project, "RedStone").await,
            ["active wins"]
        );

        // Clear RedStone's label: only the retired agent carries it now.
        register(&ctx, &project, "RedStone", Some("")).await;
        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "retired only",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect_err("only a retired agent carries the label");
        assert_eq!(error_type(&error), "AGENT_RETIRED");
        assert_eq!(error_data(&error)["agent_name"], "GreenCastle");

        deregister_agent(
            &ctx,
            project.clone(),
            "PurpleHill".to_string(),
            token(&leaving),
            None,
            None,
            None,
        )
        .await
        .expect("deregister PurpleHill");
        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "deregistered only",
                json!({"to": [{"display_name": "Auditor"}]}),
            ),
        )
        .await
        .expect_err("only a deregistered agent carries the label");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
    });
}

/// Criterion 9: one failing selector stops the whole message.
#[test]
fn one_failing_selector_writes_nothing() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;
        register(&ctx, &project, "GreenCastle", None).await;

        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "partial",
                json!({
                    "to": ["GreenCastle", {"display_name": "Reviewer"}],
                    "cc": [{"display_name": "Release Captain"}],
                }),
            ),
        )
        .await
        .expect_err("one selector fails");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        for agent in ["RedStone", "GreenCastle"] {
            assert_inbox_empty(&ctx, &project, agent).await;
        }
    });
}

/// Criterion 10: an agent named by a string and by selectors receives the
/// message once, in the first list that names it.
#[test]
fn an_agent_named_twice_receives_once() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;

        let sent = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "once",
                json!({
                    "to": ["RedStone", {"display_name": "Reviewer"}],
                    "cc": [{"name": "RedStone"}],
                    "bcc": [{"display_name": "reviewer"}],
                }),
            ),
        )
        .await
        .expect("a repeated agent is kept once");
        let payload = &sent["deliveries"][0]["payload"];
        assert_eq!(names(&payload["to"]), ["RedStone"]);
        assert_eq!(names(&payload["cc"]), Vec::<&str>::new());
        assert_eq!(inbox_subjects(&ctx, &project, "RedStone").await, ["once"]);
    });
}

/// Criterion 11: a relabel after delivery redirects nothing.
#[test]
fn relabeling_after_delivery_changes_no_delivery() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;
        register(&ctx, &project, "GreenCastle", None).await;

        let sent = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "before",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect("deliver by label");
        let message_id = sent["deliveries"][0]["payload"]["id"].clone();

        // The label moves from RedStone to GreenCastle.
        register(&ctx, &project, "RedStone", Some("Archivist")).await;
        register(&ctx, &project, "GreenCastle", Some("Reviewer")).await;

        assert_eq!(inbox_subjects(&ctx, &project, "RedStone").await, ["before"]);
        assert_inbox_empty(&ctx, &project, "GreenCastle").await;

        // A reply to the delivered message goes to its stable sender, and a
        // new send by the label reaches the new holder only.
        let replied = reply(
            &ctx,
            json!({
                "project_key": project,
                "message_id": message_id,
                "sender_name": "RedStone",
                "body_md": "ack",
            }),
        )
        .await
        .expect("reply to the delivered message");
        assert_eq!(
            names(&replied["deliveries"][0]["payload"]["to"]),
            ["BlueLake"]
        );
        send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "after",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect("deliver by the moved label");
        assert_eq!(inbox_subjects(&ctx, &project, "RedStone").await, ["before"]);
        assert_eq!(
            inbox_subjects(&ctx, &project, "GreenCastle").await,
            ["after"]
        );
    });
}

/// Criterion 12: a display name finds no agent in another project, and a
/// recipient's contact policy applies to a display-name recipient as it does
/// to the stable name.
#[test]
fn project_scope_and_contact_policy_still_apply() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", Some("Reviewer")).await;

        let other = format!("{project}-other");
        ensure_project(&ctx, other.clone(), None)
            .await
            .expect("ensure the other project");
        register(&ctx, &other, "GreenCastle", Some("Release Captain")).await;

        let error = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "other project",
                json!({"to": [{"display_name": "Release Captain"}]}),
            ),
        )
        .await
        .expect_err("the label belongs to another project");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        assert_inbox_empty(&ctx, &other, "GreenCastle").await;

        mcp_agent_mail_tools::contacts::set_contact_policy(
            &ctx,
            project.clone(),
            "RedStone".to_string(),
            "block_all".to_string(),
        )
        .await
        .expect("block all contact");
        let by_name = send(
            &ctx,
            send_arguments(&project, "BlueLake", "blocked", json!({"to": ["RedStone"]})),
        )
        .await
        .expect_err("the recipient blocks all contact");
        let by_label = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "blocked",
                json!({"to": [{"display_name": "Reviewer"}]}),
            ),
        )
        .await
        .expect_err("the recipient blocks all contact, by label too");
        assert_eq!(error_type(&by_label), error_type(&by_name));
        assert_eq!(error_type(&by_label), "CONTACT_BLOCKED");
        assert_inbox_empty(&ctx, &project, "RedStone").await;
    });
}

/// Criterion 2 for `reply_message`: a reply accepts selectors.
#[test]
fn a_reply_accepts_recipient_selectors() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", None).await;
        register(&ctx, &project, "RedStone", None).await;
        register(&ctx, &project, "GreenCastle", Some("Night Reviewer")).await;

        let sent = send(
            &ctx,
            send_arguments(
                &project,
                "BlueLake",
                "question",
                json!({"to": ["RedStone"]}),
            ),
        )
        .await
        .expect("send");
        let message_id = sent["deliveries"][0]["payload"]["id"].clone();

        let replied = reply(
            &ctx,
            json!({
                "project_key": project,
                "message_id": message_id,
                "sender_name": "RedStone",
                "body_md": "answer",
                "to": [{"name": "BlueLake"}],
                "cc": [{"display_name": "night reviewer"}],
            }),
        )
        .await
        .expect("a reply with selectors");
        let payload = &replied["deliveries"][0]["payload"];
        assert_eq!(names(&payload["to"]), ["BlueLake"]);
        assert_eq!(names(&payload["cc"]), ["GreenCastle"]);
        assert_eq!(inbox_subjects(&ctx, &project, "GreenCastle").await.len(), 1);

        let error = reply(
            &ctx,
            json!({
                "project_key": project,
                "message_id": message_id,
                "sender_name": "RedStone",
                "body_md": "answer",
                "to": [{"display_name": "Release Captain"}],
            }),
        )
        .await
        .expect_err("no agent carries the label");
        assert_eq!(error_type(&error), "RECIPIENT_DISPLAY_NAME_NOT_FOUND");
        assert_eq!(inbox_subjects(&ctx, &project, "BlueLake").await.len(), 1);
    });
}

/// Criterion 13: `whois` by display name returns every active holder and an
/// empty list for an unused label; `whois` by stable name is unchanged.
#[test]
fn whois_looks_up_a_display_name() {
    run_with_project(|ctx, project| async move {
        register(&ctx, &project, "BlueLake", Some("Reviewer")).await;
        register(&ctx, &project, "RedStone", Some("reviewer")).await;
        let retiring = register(&ctx, &project, "GreenCastle", Some("Reviewer")).await;
        retire_agent(
            &ctx,
            project.clone(),
            "GreenCastle".to_string(),
            token(&retiring),
            None,
            None,
            None,
        )
        .await
        .expect("retire GreenCastle");

        let found = whois(
            &ctx,
            json!({
                "project_key": project,
                "display_name": " REVIEWER ",
                "include_recent_commits": false,
            }),
        )
        .await
        .expect("whois by display name");
        assert_eq!(found["display_name"], "REVIEWER");
        let matches = found["matches"].as_array().expect("matches");
        let holders: Vec<&str> = matches
            .iter()
            .filter_map(|profile| profile["name"].as_str())
            .collect();
        assert_eq!(holders, ["BlueLake", "RedStone"]);

        // Each match is the profile whois returns for that agent.
        let by_name = json!({
            "project_key": project,
            "agent_name": "BlueLake",
            "include_recent_commits": false,
        });
        let wrapped = whois(&ctx, by_name.clone()).await.expect("whois by name");
        let unwrapped = call(&Whois, &ctx, by_name).await.expect("the wrapped tool");
        assert_eq!(wrapped, unwrapped);
        assert_eq!(matches[0], wrapped);

        let unused = whois(
            &ctx,
            json!({"project_key": project, "display_name": "Release Captain"}),
        )
        .await
        .expect("an unused label is no error");
        assert_eq!(unused["matches"], json!([]));

        let error = whois(
            &ctx,
            json!({
                "project_key": project,
                "agent_name": "BlueLake",
                "display_name": "Reviewer",
            }),
        )
        .await
        .expect_err("agent_name and display_name exclude each other");
        assert_eq!(error_type(&error), "INVALID_ARGUMENT");
    });
}

/// Criterion 15, schema: the definitions advertise the additional forms and
/// leave everything else as the wrapped tools define it.
#[test]
fn the_schemas_advertise_the_additional_forms() {
    for (wrapped, plain) in [
        (
            SelectorRecipients(SendMessage).definition(),
            SendMessage.definition(),
        ),
        (
            SelectorRecipients(ReplyMessage).definition(),
            ReplyMessage.definition(),
        ),
    ] {
        assert_eq!(wrapped.name, plain.name);
        let mut expected = plain.input_schema.clone();
        for field in ["to", "cc", "bcc"] {
            let list = &wrapped.input_schema["properties"][field];
            assert_eq!(
                list["items"]["type"],
                json!(["string", "object"]),
                "{}.{field}",
                wrapped.name
            );
            assert_eq!(list["items"]["additionalProperties"], json!(false));
            assert_eq!(
                list["type"], plain.input_schema["properties"][field]["type"],
                "the list itself is unchanged"
            );
            expected["properties"][field]["items"] = list["items"].clone();
        }
        assert_eq!(wrapped.input_schema, expected, "{}", wrapped.name);
        assert_eq!(
            wrapped.input_schema["required"],
            plain.input_schema["required"]
        );
        let description = wrapped.description.expect("description");
        let original = plain.description.expect("description");
        assert!(description.starts_with(&original));
        assert!(description.contains("RECIPIENT_DISPLAY_NAME_AMBIGUOUS"));
    }

    let wrapped = WhoisByDisplayName(Whois).definition();
    let plain = Whois.definition();
    let required = names(&wrapped.input_schema["required"]);
    assert!(required.contains(&"project_key"));
    assert!(!required.contains(&"agent_name"));
    assert!(names(&plain.input_schema["required"]).contains(&"agent_name"));
    assert_eq!(
        wrapped.input_schema["properties"]["display_name"]["type"],
        json!(["string", "null"])
    );
    assert_eq!(
        wrapped.input_schema["properties"]["agent_name"],
        plain.input_schema["properties"]["agent_name"]
    );
}
