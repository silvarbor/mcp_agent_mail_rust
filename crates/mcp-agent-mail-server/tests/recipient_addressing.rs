//! The server registers `send_message`, `reply_message` and `whois` with
//! explicit recipient addressing: `docs/SPEC-recipient-addressing.md`.

use fastmcp::{Cx, ListToolsParams, McpContext, Tool};
use serde_json::{Value, json};

fn registered_tools() -> Vec<Tool> {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let mut config = mcp_agent_mail_core::Config::from_env();
    config.database_url = format!(
        "sqlite://{}",
        tmp.path().join("recipient-addressing.sqlite3").display()
    );
    config.storage_root = tmp.path().join("archive");
    config.tool_filter.enabled = false;
    let router = mcp_agent_mail_server::build_server(&config).into_router();
    router
        .handle_tools_list(
            &McpContext::new(Cx::for_testing(), 1),
            ListToolsParams::default(),
            None,
        )
        .expect("tools/list")
        .tools
}

fn tool<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("{name} is registered"))
}

fn required(tool: &Tool) -> Vec<&str> {
    tool.input_schema["required"]
        .as_array()
        .map(|names| names.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

#[test]
fn the_server_advertises_recipient_selectors_and_whois_by_display_name() {
    let tools = registered_tools();

    for name in ["send_message", "reply_message"] {
        let tool = tool(&tools, name);
        for field in ["to", "cc", "bcc"] {
            let items = &tool.input_schema["properties"][field]["items"];
            assert_eq!(items["type"], json!(["string", "object"]), "{name}.{field}");
            assert_eq!(
                items["properties"],
                json!({
                    "name": { "type": "string" },
                    "display_name": { "type": "string" },
                }),
                "{name}.{field}"
            );
            assert_eq!(items["additionalProperties"], json!(false));
            assert_eq!(items["minProperties"], json!(1));
            assert_eq!(items["maxProperties"], json!(1));
        }
        let description = tool.description.as_deref().expect("description");
        assert!(
            description.contains("RECIPIENT_DISPLAY_NAME_NOT_FOUND"),
            "{name}"
        );
    }
    assert!(required(tool(&tools, "send_message")).contains(&"to"));
    assert!(!required(tool(&tools, "reply_message")).contains(&"to"));

    let whois = tool(&tools, "whois");
    assert_eq!(
        whois.input_schema["properties"]["display_name"]["type"],
        json!(["string", "null"])
    );
    let whois_required = required(whois);
    assert!(whois_required.contains(&"project_key"));
    assert!(!whois_required.contains(&"agent_name"));
    assert!(
        whois
            .description
            .as_deref()
            .expect("description")
            .contains("Lookup by display name")
    );
}
