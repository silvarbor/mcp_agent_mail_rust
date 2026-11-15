//! `am mail send` and `am mail reply` with display-name recipients, in the
//! CLI's own process: no server is reachable, so the CLI resolves each label
//! itself and then sends through the same mailbox
//! (`docs/SPEC-recipient-addressing.md`, criterion 14).

use super::{TestEnv, am_bin};
use serde_json::Value;
use std::process::{Command, Output};

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn mail_send_and_reply_resolve_display_names_without_a_server() {
    let env = TestEnv::new();
    let project = env.hostile_repo.to_str().expect("project path");
    let run = |args: &[&str]| -> Output {
        Command::new(am_bin())
            .env_clear()
            .envs(env.isolated_env())
            .current_dir(project)
            .args(args)
            .output()
            .expect("run am")
    };
    let succeed = |args: &[&str]| -> Output {
        let output = run(args);
        assert!(output.status.success(), "{args:?}: {}", stderr(&output));
        output
    };
    let subjects = |agent: &str| -> Vec<(i64, String)> {
        let output = succeed(&[
            "robot",
            "inbox",
            "--project",
            project,
            "--agent",
            agent,
            "--format",
            "json",
        ]);
        let inbox: Value = serde_json::from_slice(&output.stdout).expect("inbox JSON");
        let mut subjects: Vec<(i64, String)> = inbox["inbox"]
            .as_array()
            .expect("inbox list")
            .iter()
            .map(|message| {
                (
                    message["id"].as_i64().expect("message id"),
                    message["subject"].as_str().expect("subject").to_string(),
                )
            })
            .collect();
        subjects.sort();
        subjects
    };

    for (name, label) in [
        ("BlueLake", "Captain"),
        ("RedStone", "Team A, B"),
        ("GreenCastle", "Reviewer"),
        ("PurpleHill", "reviewer"),
    ] {
        succeed(&[
            "agents",
            "register",
            "--project",
            project,
            "--name",
            name,
            "--program",
            "codex-cli",
            "--model",
            "test",
            "--display-name",
            label,
            "--json",
        ]);
        succeed(&[
            "contacts",
            "policy",
            "--project",
            project,
            "--agent",
            name,
            "open",
        ]);
    }

    // One label, taken whole: the comma is part of it. No --to is needed.
    succeed(&[
        "mail",
        "send",
        "--project",
        project,
        "--from",
        "BlueLake",
        "--to-display-name",
        "team a, b",
        "--subject",
        "by label",
        "--body",
        "sent by display name",
        "--json",
    ]);
    let delivered = subjects("RedStone");
    assert_eq!(
        delivered
            .iter()
            .map(|(_, subject)| subject.as_str())
            .collect::<Vec<_>>(),
        ["by label"]
    );
    let message_id = delivered[0].0.to_string();

    // Two active agents carry "Reviewer": nothing is sent, both are named.
    let ambiguous = run(&[
        "mail",
        "send",
        "--project",
        project,
        "--from",
        "BlueLake",
        "--to",
        "RedStone",
        "--to-display-name",
        "Reviewer",
        "--subject",
        "ambiguous",
        "--body",
        "must not send",
        "--json",
    ]);
    assert!(!ambiguous.status.success());
    let error = stderr(&ambiguous);
    assert!(
        error.contains("carry the display name 'Reviewer'"),
        "{error}"
    );
    assert!(error.contains("GreenCastle, PurpleHill"), "{error}");

    let absent = run(&[
        "mail",
        "send",
        "--project",
        project,
        "--from",
        "BlueLake",
        "--to-display-name",
        "Release Captain",
        "--subject",
        "absent",
        "--body",
        "must not send",
        "--json",
    ]);
    assert!(!absent.status.success());
    let error = stderr(&absent);
    assert!(
        error.contains("carries the display name 'Release Captain'"),
        "{error}"
    );

    succeed(&[
        "mail",
        "reply",
        "--project",
        project,
        "--from",
        "RedStone",
        "--message-id",
        &message_id,
        "--body",
        "reply by display name",
        "--to-display-name",
        "captain",
        "--json",
    ]);

    // The failed sends reached nobody; the reply reached the label's holder.
    assert_eq!(subjects("RedStone").len(), 1);
    for agent in ["GreenCastle", "PurpleHill"] {
        assert_eq!(subjects(agent), Vec::new(), "{agent} received nothing");
    }
    assert_eq!(
        subjects("BlueLake")
            .iter()
            .map(|(_, subject)| subject.as_str())
            .collect::<Vec<_>>(),
        ["Re: by label"]
    );
}
