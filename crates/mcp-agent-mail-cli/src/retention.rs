//! Explicit, resumable offline retention. Expired archive files are moved, never deleted.

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use mcp_agent_mail_db::mail_retention::{RetentionState, STATE_FILE};
use mcp_agent_mail_db::{CanonicalDbConn, DbConn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlmodel_core::{Row, Value};

use crate::{CliError, CliResult};

const MANIFEST: &str = "manifest.json";
const SNAPSHOT: &str = "before.sqlite3";

#[derive(Debug, Serialize, Deserialize)]
struct ArchiveFile {
    relative: PathBuf,
    sha256: String,
    kind: ArchiveFileKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ArchiveFileKind {
    Message,
    ThreadDigest,
}

fn archive_file_kind(relative: &Path) -> Option<ArchiveFileKind> {
    if relative
        .extension()
        .is_none_or(|extension| extension != "md")
    {
        return None;
    }
    let parts = relative.iter().collect::<Vec<_>>();
    if parts.len() == 3 && parts[0] == "messages" && parts[1] == "threads" {
        return Some(ArchiveFileKind::ThreadDigest);
    }
    let date_offset = if parts.len() == 4 && parts[0] == "messages" {
        1
    } else if parts.len() == 6
        && parts[0] == "agents"
        && (parts[2] == "inbox" || parts[2] == "outbox")
    {
        3
    } else {
        return None;
    };
    let year = parts[date_offset].to_str()?;
    let month = parts[date_offset + 1].to_str()?;
    (year.len() == 4
        && year.bytes().all(|byte| byte.is_ascii_digit())
        && month.len() == 2
        && month.bytes().all(|byte| byte.is_ascii_digit()))
    .then_some(ArchiveFileKind::Message)
}

#[derive(Debug, Serialize, Deserialize)]
struct Plan {
    version: u32,
    database: PathBuf,
    storage_root: PathBuf,
    project_id: i64,
    project_slug: String,
    before_micros: i64,
    message_id_floor: i64,
    messages_before: i64,
    expired_ids: BTreeSet<i64>,
    files: Vec<ArchiveFile>,
    snapshot_sha256: String,
}

fn failure(error: impl std::fmt::Display) -> CliError {
    CliError::Other(error.to_string())
}

fn digest(path: &Path) -> CliResult<String> {
    use std::io::Read;
    let mut file = crate::doctor::platform::open_regular_file_no_follow(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let length = file.read(&mut buffer)?;
        if length == 0 {
            break;
        }
        hash.update(&buffer[..length]);
    }
    Ok(hex::encode(hash.finalize()))
}

fn publish_json(path: &Path, value: &impl Serialize) -> CliResult<()> {
    crate::ensure_real_file_target_path(path, "retention record")?;
    let mut staged = tempfile::NamedTempFile::new_in(
        path.parent()
            .ok_or_else(|| failure("missing record parent"))?,
    )?;
    serde_json::to_writer_pretty(&mut staged, value).map_err(failure)?;
    staged.write_all(b"\n")?;
    staged.as_file().sync_all()?;
    staged.persist(path).map_err(failure)?;
    sync_directory(
        path.parent()
            .ok_or_else(|| failure("missing record parent"))?,
    )?;
    Ok(())
}

fn sync_directory(path: &Path) -> CliResult<()> {
    #[cfg(unix)]
    std::fs::File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

fn rows(conn: &CanonicalDbConn, sql: &str) -> CliResult<Vec<Vec<Value>>> {
    Ok(conn
        .query_sync(sql, &[])
        .map_err(failure)?
        .iter()
        .map(|row| row.iter().map(|(_, value)| value.clone()).collect())
        .collect())
}

fn number(row: &Row, name: &str) -> CliResult<i64> {
    row.get_named(name).map_err(failure)
}

fn inspect_snapshot(path: &Path) -> CliResult<CanonicalDbConn> {
    let conn = CanonicalDbConn::open_file(crate::sqlite_snapshot_path_text(
        path,
        "retention",
        "snapshot",
    )?)
    .map_err(failure)?;
    let integrity = conn
        .query_sync("PRAGMA integrity_check", &[])
        .map_err(failure)?;
    if integrity.len() != 1 || integrity[0].get_as::<String>(0).map_err(failure)? != "ok" {
        return Err(failure("retention snapshot failed full integrity check"));
    }
    if !conn
        .query_sync("PRAGMA foreign_key_check", &[])
        .map_err(failure)?
        .is_empty()
    {
        return Err(failure("retention snapshot failed foreign key check"));
    }
    Ok(conn)
}

fn collect_files(
    root: &Path,
    slug: &str,
    ids: &BTreeSet<i64>,
    known_ids: &BTreeSet<i64>,
) -> CliResult<Vec<ArchiveFile>> {
    let project_root = root.join("projects").join(slug);
    if !project_root.exists() {
        return Ok(Vec::new());
    }
    crate::ensure_real_directory_tree(&project_root, "retention project")?;
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(&project_root).follow_links(false) {
        let entry = entry.map_err(failure)?;
        if entry.file_type().is_symlink() {
            return Err(failure(format!(
                "retention refuses symlink {}",
                entry.path().display()
            )));
        }
        let relative_project = entry.path().strip_prefix(&project_root).map_err(failure)?;
        let Some(kind) = archive_file_kind(relative_project) else {
            continue;
        };
        if !entry.file_type().is_file()
            || entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        if kind == ArchiveFileKind::ThreadDigest {
            files.push(ArchiveFile {
                relative: entry
                    .path()
                    .strip_prefix(root)
                    .map_err(failure)?
                    .to_path_buf(),
                sha256: digest(entry.path())?,
                kind,
            });
            continue;
        }
        let (metadata, _) =
            mcp_agent_mail_storage::read_message_file(entry.path()).map_err(failure)?;
        let id = metadata
            .get("id")
            .and_then(serde_json::Value::as_i64)
            .filter(|id| *id > 0)
            .ok_or_else(|| {
                failure(format!(
                    "message has no valid ID: {}",
                    entry.path().display()
                ))
            })?;
        if !known_ids.contains(&id) {
            return Err(failure(format!(
                "archive message {id} is absent from the selected project's database; reconcile before retention"
            )));
        }
        if ids.contains(&id) {
            files.push(ArchiveFile {
                relative: entry
                    .path()
                    .strip_prefix(root)
                    .map_err(failure)?
                    .to_path_buf(),
                sha256: digest(entry.path())?,
                kind,
            });
        }
    }
    files.sort_by(|left, right| left.relative.cmp(&right.relative));
    Ok(files)
}

fn create_plan(
    database: &Path,
    root: &Path,
    project: &str,
    before: i64,
    run_dir: &Path,
) -> CliResult<Plan> {
    if std::fs::symlink_metadata(run_dir).is_ok() {
        return Err(failure(
            "preview requires a fresh run directory; apply uses an existing manifest",
        ));
    }
    crate::ensure_real_directory_tree(
        run_dir
            .parent()
            .ok_or_else(|| failure("missing run parent"))?,
        "retention run parent",
    )?;
    std::fs::create_dir(run_dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(run_dir, std::fs::Permissions::from_mode(0o700))?;
    }
    let snapshot = run_dir.join(SNAPSHOT);
    crate::vacuum_live_franken_sqlite_into_snapshot(database, &snapshot, "mail retention preview")?;
    let conn = inspect_snapshot(&snapshot)?;
    let projects = conn
        .query_sync(
            "SELECT id, slug FROM projects WHERE slug = ? OR human_key = ?",
            &[
                Value::Text(project.to_owned()),
                Value::Text(project.to_owned()),
            ],
        )
        .map_err(failure)?;
    if projects.len() != 1 {
        return Err(failure("retention requires one exact, existing project"));
    }
    let project_id = number(&projects[0], "id")?;
    let slug: String = projects[0].get_named("slug").map_err(failure)?;
    if Path::new(&slug).components().count() != 1
        || !matches!(
            Path::new(&slug).components().next(),
            Some(Component::Normal(_))
        )
    {
        return Err(failure("unsafe project slug"));
    }
    let expired_ids = conn
        .query_sync(
            "SELECT id FROM messages WHERE project_id = ? AND created_ts < ? ORDER BY id",
            &[Value::BigInt(project_id), Value::BigInt(before)],
        )
        .map_err(failure)?
        .iter()
        .map(|row| number(row, "id"))
        .collect::<CliResult<BTreeSet<_>>>()?;
    let totals = conn
        .query_sync(
            "SELECT COUNT(*) AS total, COALESCE(MAX(id), 0) AS floor FROM messages",
            &[],
        )
        .map_err(failure)?;
    let allocator = conn
        .query_sync(
            "SELECT COALESCE(MAX(seq), 0) AS floor FROM sqlite_sequence WHERE name = 'messages'",
            &[],
        )
        .map_err(failure)?;
    let prior_floor = mcp_agent_mail_db::mail_retention::require_complete(root)
        .map_err(failure)?
        .unwrap_or(0);
    let floor = number(&totals[0], "floor")?
        .max(number(&allocator[0], "floor")?)
        .max(prior_floor);
    let known_ids = conn
        .query_sync(
            "SELECT id FROM messages WHERE project_id = ?",
            &[Value::BigInt(project_id)],
        )
        .map_err(failure)?
        .iter()
        .map(|row| number(row, "id"))
        .collect::<CliResult<BTreeSet<_>>>()?;
    let files = collect_files(root, &slug, &expired_ids, &known_ids)?;
    let plan = Plan {
        version: 2,
        database: database.to_path_buf(),
        storage_root: root.to_path_buf(),
        project_id,
        project_slug: slug,
        before_micros: before,
        message_id_floor: floor,
        messages_before: number(&totals[0], "total")?,
        expired_ids,
        files,
        snapshot_sha256: digest(&snapshot)?,
    };
    publish_json(&run_dir.join(MANIFEST), &plan)?;
    Ok(plan)
}

fn verify_remaining(
    before: &CanonicalDbConn,
    after: &CanonicalDbConn,
    plan: &Plan,
) -> CliResult<()> {
    let tables = before.query_sync("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name", &[]).map_err(failure)?;
    for table in tables {
        let name: String = table.get_named("name").map_err(failure)?;
        if name.starts_with("fts_")
            || matches!(
                name.as_str(),
                "messages"
                    | "message_recipients"
                    | "project_mailbox_deliveries"
                    | "project_mailbox_receipts"
                    | "inbox_delivery_events"
                    | "message_delivery_signal_receipts"
                    | "inbox_stats"
                    | "lexical_change_clock"
            )
        {
            continue;
        }
        let sql = format!(
            "SELECT * FROM \"{}\" ORDER BY rowid",
            name.replace('"', "\"\"")
        );
        if rows(before, &sql)? != rows(after, &sql)? {
            return Err(failure(format!("retention changed protected table {name}")));
        }
    }
    let retained = format!(
        "NOT (project_id = {} AND created_ts < {})",
        plan.project_id, plan.before_micros
    );
    if rows(
        before,
        &format!("SELECT * FROM messages WHERE {retained} ORDER BY id"),
    )? != rows(after, "SELECT * FROM messages ORDER BY id")?
    {
        return Err(failure("retention changed retained message records"));
    }
    for (table, order) in [
        ("message_recipients", "message_id, agent_id"),
        ("project_mailbox_deliveries", "message_id"),
        ("project_mailbox_receipts", "message_id, agent_id"),
        ("inbox_delivery_events", "seq"),
        (
            "message_delivery_signal_receipts",
            "message_id, agent_id, delivery_route",
        ),
    ] {
        let source = format!(
            "SELECT * FROM {table} WHERE message_id IN (SELECT id FROM messages WHERE {retained}) ORDER BY {order}"
        );
        let target = format!("SELECT * FROM {table} ORDER BY {order}");
        if rows(before, &source)? != rows(after, &target)? {
            return Err(failure(format!("retention changed retained {table}")));
        }
    }
    let floor = after
        .query_sync(
            "SELECT COALESCE(MAX(seq), 0) AS floor FROM sqlite_sequence WHERE name = 'messages'",
            &[],
        )
        .map_err(failure)?;
    if number(&floor[0], "floor")? < plan.message_id_floor {
        return Err(failure("message allocator floor regressed"));
    }
    Ok(())
}

fn expire_rows(conn: &DbConn, plan: &Plan) -> CliResult<()> {
    conn.execute_raw("BEGIN IMMEDIATE").map_err(failure)?;
    let result = (|| {
        let predicate = format!(
            "SELECT id FROM messages WHERE project_id = {} AND created_ts < {}",
            plan.project_id, plan.before_micros
        );
        for table in [
            "project_mailbox_receipts",
            "project_mailbox_deliveries",
            "message_delivery_signal_receipts",
            "inbox_delivery_events",
            "message_recipients",
        ] {
            conn.execute_raw(&format!(
                "DELETE FROM {table} WHERE message_id IN ({predicate})"
            ))
            .map_err(failure)?;
        }
        conn.execute_raw(&format!(
            "DELETE FROM messages WHERE project_id = {} AND created_ts < {}",
            plan.project_id, plan.before_micros
        ))
        .map_err(failure)?;
        conn.execute_raw("DELETE FROM inbox_stats")
            .map_err(failure)?;
        conn.execute_raw("INSERT INTO inbox_stats (agent_id, total_count, unread_count, ack_pending_count, last_message_ts) SELECT r.agent_id, COUNT(*), SUM(CASE WHEN r.read_ts IS NULL THEN 1 ELSE 0 END), SUM(CASE WHEN m.ack_required = 1 AND r.ack_ts IS NULL THEN 1 ELSE 0 END), MAX(m.created_ts) FROM message_recipients r JOIN messages m ON m.id = r.message_id GROUP BY r.agent_id").map_err(failure)?;
        conn.execute_raw("COMMIT").map_err(failure)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = conn.execute_raw("ROLLBACK");
    }
    result
}

fn apply_plan(plan: &Plan, run_dir: &Path) -> CliResult<()> {
    mcp_agent_mail_storage::require_drained_archive_backlog(&mcp_agent_mail_core::Config {
        storage_root: plan.storage_root.clone(),
        ..Default::default()
    })
    .map_err(failure)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if std::fs::metadata(run_dir)?.dev() != std::fs::metadata(&plan.storage_root)?.dev() {
            return Err(failure(
                "retention recovery must be on the same filesystem for reversible archive moves",
            ));
        }
    }
    if digest(&run_dir.join(SNAPSHOT))? != plan.snapshot_sha256 {
        return Err(failure("offline recovery snapshot hash changed"));
    }
    let before = inspect_snapshot(&run_dir.join(SNAPSHOT))?;
    let expected_ids = before
        .query_sync(
            "SELECT id FROM messages WHERE project_id = ? AND created_ts < ? ORDER BY id",
            &[
                Value::BigInt(plan.project_id),
                Value::BigInt(plan.before_micros),
            ],
        )
        .map_err(failure)?
        .iter()
        .map(|row| number(row, "id"))
        .collect::<CliResult<BTreeSet<_>>>()?;
    if expected_ids != plan.expired_ids
        || plan.before_micros > (Utc::now() - Duration::hours(24)).timestamp_micros()
    {
        return Err(failure(
            "retention manifest IDs or cutoff disagree with the recovery snapshot",
        ));
    }
    let state =
        mcp_agent_mail_db::mail_retention::read_state(&plan.storage_root).map_err(failure)?;
    if let Some(state) = &state {
        if !state.complete && state.run_dir != run_dir {
            return Err(failure("another retention operation is unfinished"));
        }
        if state.complete && state.run_dir == run_dir {
            return commit_archive_plan(plan);
        }
    }
    if state.as_ref().is_none_or(|state| state.complete) {
        let current_path = run_dir.join(format!(
            "admission-{}.sqlite3",
            Utc::now().timestamp_micros()
        ));
        crate::vacuum_live_franken_sqlite_into_snapshot(
            &plan.database,
            &current_path,
            "retention admission",
        )?;
        let current = inspect_snapshot(&current_path)?;
        let tables = before
            .query_sync(
                "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name",
                &[],
            )
            .map_err(failure)?;
        for table in tables {
            let name: String = table.get_named("name").map_err(failure)?;
            if name.starts_with("fts_") {
                continue;
            }
            let sql = format!(
                "SELECT * FROM \"{}\" ORDER BY rowid",
                name.replace('"', "\"\"")
            );
            if rows(&before, &sql)? != rows(&current, &sql)? {
                return Err(failure(format!(
                    "database changed since preview ({name}); create a fresh plan"
                )));
            }
        }
        let known_ids = before
            .query_sync(
                "SELECT id FROM messages WHERE project_id = ?",
                &[Value::BigInt(plan.project_id)],
            )
            .map_err(failure)?
            .iter()
            .map(|row| number(row, "id"))
            .collect::<CliResult<BTreeSet<_>>>()?;
        let current_files = collect_files(
            &plan.storage_root,
            &plan.project_slug,
            &plan.expired_ids,
            &known_ids,
        )?;
        if serde_json::to_value(&current_files).map_err(failure)?
            != serde_json::to_value(&plan.files).map_err(failure)?
        {
            return Err(failure(
                "archive changed since preview; create a fresh plan",
            ));
        }
    }
    // Validate every path before publishing the interruption guard or moving anything.
    for file in &plan.files {
        if file
            .relative
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
            || !file
                .relative
                .starts_with(Path::new("projects").join(&plan.project_slug))
        {
            return Err(failure("manifest contains an unsafe archive path"));
        }
        let project_relative = file
            .relative
            .strip_prefix(Path::new("projects").join(&plan.project_slug))
            .map_err(failure)?;
        if archive_file_kind(project_relative) != Some(file.kind) {
            return Err(failure(
                "manifest must select dated message files or derived thread digests",
            ));
        }
        let source = plan.storage_root.join(&file.relative);
        let target = run_dir.join("archive").join(&file.relative);
        crate::ensure_real_directory_tree(
            source
                .parent()
                .ok_or_else(|| failure("missing archive parent"))?,
            "retention source",
        )?;
        crate::ensure_real_directory_tree(
            target
                .parent()
                .ok_or_else(|| failure("missing recovery parent"))?,
            "retention recovery",
        )?;
        let selected = if source.exists() && !target.exists() {
            source
        } else if !source.exists() && target.exists() {
            target
        } else {
            return Err(failure("archive source/recovery occupancy is ambiguous"));
        };
        if digest(&selected)? != file.sha256 {
            return Err(failure(format!(
                "archive changed since preview: {}",
                file.relative.display()
            )));
        }
    }
    let marker = plan.storage_root.join(STATE_FILE);
    let mut state = RetentionState {
        version: 1,
        complete: false,
        message_id_floor: plan.message_id_floor,
        run_dir: run_dir.to_path_buf(),
    };
    publish_json(&marker, &state)?;
    for file in &plan.files {
        let source = plan.storage_root.join(&file.relative);
        let target = run_dir.join("archive").join(&file.relative);
        if source.exists() {
            std::fs::rename(&source, &target)?;
        }
    }
    let mut changed_directories = BTreeSet::new();
    for file in &plan.files {
        if let Some(parent) = plan.storage_root.join(&file.relative).parent() {
            changed_directories.insert(parent.to_path_buf());
        }
        let target = run_dir.join("archive").join(&file.relative);
        for parent in target
            .ancestors()
            .skip(1)
            .take_while(|parent| parent.starts_with(run_dir))
        {
            changed_directories.insert(parent.to_path_buf());
        }
    }
    for directory in changed_directories {
        sync_directory(&directory)?;
    }
    let search = plan.storage_root.join("search_index");
    let saved_search = run_dir.join("search_index");
    if search.exists() {
        crate::ensure_real_directory_tree(&search, "retention search index")?;
        if saved_search.exists() {
            return Err(failure("search index recovery target is occupied"));
        }
        std::fs::rename(&search, &saved_search)?;
        sync_directory(&plan.storage_root)?;
        sync_directory(run_dir)?;
    }
    let conn = DbConn::open_file(crate::sqlite_snapshot_path_text(
        &plan.database,
        "retention",
        "database",
    )?)
    .map_err(failure)?;
    expire_rows(&conn, plan)?;
    drop(conn);
    mcp_agent_mail_db::pool::wal_checkpoint_truncate_path(&plan.database).map_err(failure)?;
    // Keep a fresh independent physical/record check for each attempt, including resumes.
    let after_path = run_dir.join(format!("after-{}.sqlite3", Utc::now().timestamp_micros()));
    crate::vacuum_live_franken_sqlite_into_snapshot(
        &plan.database,
        &after_path,
        "mail retention validation",
    )?;
    let after = inspect_snapshot(&after_path)?;
    verify_remaining(&before, &after, plan)?;
    publish_json(
        &run_dir.join("result.json"),
        &serde_json::json!({
            "complete": true, "expired_messages": plan.expired_ids.len(), "archive_files_moved": plan.files.len(),
        "thread_digests_invalidated": plan.files.iter().filter(|file| file.kind == ArchiveFileKind::ThreadDigest).count(),
            "message_id_floor": plan.message_id_floor, "integrity": "ok", "relationships": "unchanged",
            "snapshot": run_dir.join(SNAPSHOT), "checked_snapshot": after_path
        }),
    )?;
    state.complete = true;
    publish_json(&marker, &state)?;
    // The active data is fully verified before the complete record can enter Git.
    // A failed history commit leaves the mailbox guarded until the same run resumes.
    if let Err(error) = commit_archive_plan(plan) {
        state.complete = false;
        publish_json(&marker, &state)?;
        return Err(error);
    }
    Ok(())
}

fn commit_archive_plan(plan: &Plan) -> CliResult<()> {
    if !plan.storage_root.join(".git").exists() {
        return Ok(());
    }
    let repo = git2::Repository::open(&plan.storage_root).map_err(failure)?;
    if let Ok(head) = repo.head() {
        let tree = head.peel_to_tree().map_err(failure)?;
        let committed_state = tree
            .get_path(Path::new(STATE_FILE))
            .ok()
            .and_then(|entry| repo.find_blob(entry.id()).ok());
        if let Some(committed_state) = committed_state {
            let current_state = mcp_agent_mail_core::disk::read_regular_file_no_follow_bounded(
                &plan.storage_root.join(STATE_FILE),
                16_384,
            )?;
            let mut already_absent = true;
            for file in &plan.files {
                match tree.get_path(&file.relative) {
                    Ok(_) => {
                        already_absent = false;
                        break;
                    }
                    Err(error) if error.code() == git2::ErrorCode::NotFound => (),
                    Err(error) => return Err(failure(error)),
                }
            }
            if already_absent && committed_state.content() == current_state {
                return Ok(());
            }
        }
    }
    let mut paths = plan
        .files
        .iter()
        .map(|file| {
            file.relative
                .to_str()
                .ok_or_else(|| failure("archive commit path must be UTF-8"))
        })
        .collect::<CliResult<Vec<_>>>()?;
    paths.push(STATE_FILE);
    let config = mcp_agent_mail_core::Config::from_env();
    mcp_agent_mail_storage::commit_paths_with_retry(
        &plan.storage_root,
        &config,
        &format!(
            "Expire {} old messages from {} before {}",
            plan.expired_ids.len(),
            plan.project_slug,
            plan.before_micros
        ),
        &paths,
    )
    .map_err(failure)
}

pub(super) fn run(
    project: Option<&str>,
    before: Option<&str>,
    run_dir: &Path,
    apply: bool,
) -> CliResult<()> {
    let config = mcp_agent_mail_core::Config::from_env();
    let database = crate::doctor_reconstruct_db_path_from_database_url(&config.database_url)?
        .canonicalize()?;
    let root = config.storage_root.canonicalize()?;
    let run_dir = if run_dir.is_absolute() {
        run_dir.to_path_buf()
    } else {
        std::env::current_dir()?.join(run_dir)
    };
    if run_dir
        .components()
        .any(|part| matches!(part, Component::ParentDir))
        || run_dir.starts_with(&root)
    {
        return Err(failure(
            "retention recovery directory must be outside the active storage root",
        ));
    }
    // These exclusive locks also refuse dry runs while a daemon owns the mailbox.
    let _locks = crate::acquire_cli_mailbox_mutation_locks(&config.database_url, Some(&root))?;
    mcp_agent_mail_storage::require_drained_archive_backlog(&config).map_err(failure)?;
    if apply {
        crate::ensure_real_directory_tree(&run_dir, "retention run")?;
        let plan: Plan = serde_json::from_reader(
            crate::doctor::platform::open_regular_file_no_follow(&run_dir.join(MANIFEST))?,
        )
        .map_err(failure)?;
        if plan.version != 2 || plan.database != database || plan.storage_root != root {
            return Err(failure("retention manifest does not select this mailbox"));
        }
        if project.is_some() || before.is_some() {
            return Err(failure(
                "apply uses the saved manifest; omit --project and --before",
            ));
        }
        apply_plan(&plan, &run_dir)?;
        ftui_runtime::ftui_println!(
            "{}",
            serde_json::json!({"complete": true, "expired_messages": plan.expired_ids.len(), "run_dir": run_dir})
        );
    } else {
        let project = project.ok_or_else(|| failure("preview requires --project"))?;
        let cutoff = before
            .map(DateTime::parse_from_rfc3339)
            .transpose()
            .map_err(failure)?
            .map_or_else(
                || Utc::now() - Duration::hours(24),
                |time| time.with_timezone(&Utc),
            );
        if cutoff > Utc::now() - Duration::hours(24) {
            return Err(failure("retention must keep at least the last 24 hours"));
        }
        let plan = create_plan(
            &database,
            &root,
            project,
            cutoff.timestamp_micros(),
            &run_dir,
        )?;
        ftui_runtime::ftui_println!(
            "{}",
            serde_json::json!({"dry_run": true, "project": plan.project_slug,
            "before": cutoff.to_rfc3339(), "expired_messages": plan.expired_ids.len(), "retained_messages": plan.messages_before - i64::try_from(plan.expired_ids.len()).map_err(failure)?,
            "archive_files": plan.files.len(), "run_dir": run_dir})
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn retention_cli_requires_a_run_directory_and_defaults_to_preview() {
        assert!(
            crate::Cli::try_parse_from(["am", "archive", "prune", "--project", "/shared"]).is_err()
        );
        let parsed = crate::Cli::try_parse_from([
            "am",
            "archive",
            "prune",
            "--project",
            "/shared",
            "--run-dir",
            "recovery",
        ])
        .unwrap();
        assert!(matches!(
            parsed.command,
            Some(crate::Commands::Archive {
                action: crate::ArchiveCommand::Prune { apply: false, .. }
            })
        ));
        let parsed = crate::Cli::try_parse_from([
            "am",
            "archive",
            "prune",
            "--apply",
            "--run-dir",
            "recovery",
        ])
        .unwrap();
        assert!(matches!(
            parsed.command,
            Some(crate::Commands::Archive {
                action: crate::ArchiveCommand::Prune {
                    apply: true,
                    project: None,
                    before: None,
                    ..
                }
            })
        ));
    }

    // Scenario from the retention contract: all four old read/ack states expire,
    // the boundary and newer rows survive, another project survives, and an old
    // maximum ID must remain reserved after reconstruction. The canonical SQLite
    // oracle reads the entire retained rows independently of the deletion engine.
    #[test]
    fn retention_preserves_boundary_relationships_receipts_and_allocator() {
        let fixture = tempfile::tempdir().unwrap();
        let base = fixture.path().canonicalize().unwrap();
        let root = base.join("store");
        std::fs::create_dir(&root).unwrap();
        let database = root.join("mail.sqlite3");
        let pool = mcp_agent_mail_db::DbPool::new(&mcp_agent_mail_db::DbPoolConfig {
            database_url: mcp_agent_mail_core::disk::sqlite_url_from_path(&database),
            storage_root: Some(root.clone()),
            max_connections: 1,
            min_connections: 1,
            ..Default::default()
        })
        .unwrap();
        let runtime = asupersync::runtime::RuntimeBuilder::current_thread()
            .build()
            .unwrap();
        runtime.block_on(async {
            let cx = asupersync::Cx::current().unwrap();
            match pool.acquire(&cx).await {
                asupersync::Outcome::Ok(connection) => drop(connection),
                asupersync::Outcome::Err(error) => panic!("fixture initialization failed: {error}"),
                asupersync::Outcome::Cancelled(reason) => {
                    panic!("fixture initialization cancelled: {reason:?}")
                }
                asupersync::Outcome::Panicked(payload) => {
                    panic!("fixture initialization panicked: {payload}")
                }
            }
        });
        drop(pool);
        let conn = DbConn::open_file(database.to_str().unwrap()).unwrap();
        conn.execute_raw("INSERT INTO projects(id,slug,human_key,created_at) VALUES (1,'shared','/shared',0),(2,'other','/other',0);
            INSERT INTO agents(id,project_id,name,program,model,inception_ts,last_active_ts,display_name) VALUES
            (1,1,'BlueLake','test','test',0,0,'Reviewer'),(2,1,'RedStone','test','test',0,0,'Reviewer'),(3,2,'GreenLake','test','test',0,0,NULL);
            INSERT INTO agent_links(id,a_project_id,a_agent_id,b_project_id,b_agent_id,status,created_ts,updated_ts) VALUES (19,1,1,1,2,'accepted',0,0);
            INSERT INTO file_reservations(id,project_id,agent_id,path_pattern,created_ts,expires_ts) VALUES (23,1,1,'src/**',0,9999999999);
            INSERT INTO messages(id,project_id,sender_id,subject,body_md,created_ts,ack_required,recipients_json) VALUES
            (100,1,1,'old unread','α',999999,1,'{\"to\":[\"RedStone\"],\"cc\":[],\"bcc\":[]}'),
            (101,1,1,'old read','β',999999,1,'{}'),(102,1,1,'old acknowledged','γ',999999,1,'{}'),(103,1,1,'old read acknowledged','δ',999999,1,'{}'),
            (2,1,1,'boundary','keep',1000000,1,'{\"to\":[\"RedStone\"],\"cc\":[],\"bcc\":[]}'),(3,1,2,'recent','keep',1000001,0,'{}'),(4,2,3,'other old','keep',0,0,'{}');
            INSERT INTO message_recipients(message_id,agent_id,kind,read_ts,ack_ts) VALUES
            (100,2,'to',NULL,NULL),(101,2,'to',7,NULL),(102,2,'to',NULL,8),(103,2,'to',7,8),(2,2,'to',7,8),(3,1,'to',NULL,NULL),(4,3,'to',NULL,NULL);
            INSERT INTO message_delivery_signal_receipts(message_id,agent_id,delivery_route,signal_path_digest,observed_ts) VALUES (100,2,'signal','old',9),(2,2,'signal','keep',10);").unwrap();
        conn.execute_raw(
            "INSERT INTO project_mailbox_deliveries(message_id,project_id,kind,delivered_ts) VALUES
             (100,1,'to',999999),(2,1,'to',1000000),(4,2,'to',0);
             INSERT INTO project_mailbox_receipts(message_id,agent_id,read_ts,ack_ts) VALUES
             (100,2,7,8),(2,2,7,8),(4,3,NULL,NULL);",
        )
        .unwrap();
        drop(conn);
        let project_dir = root.join("projects/shared");
        std::fs::create_dir_all(&project_dir).unwrap();
        std::fs::write(
            project_dir.join("project.json"),
            r#"{"slug":"shared","human_key":"/shared","created_at":0}"#,
        )
        .unwrap();
        for name in ["BlueLake", "RedStone"] {
            let path = project_dir.join("agents").join(name);
            std::fs::create_dir_all(&path).unwrap();
            std::fs::write(path.join("profile.json"),serde_json::to_vec(&serde_json::json!({"name":name,"program":"test","model":"test","display_name":"Reviewer","inception_ts":0,"last_active_ts":0})).unwrap()).unwrap();
        }
        for (id, time) in [
            (100, 999999),
            (101, 999999),
            (102, 999999),
            (103, 999999),
            (2, 1000000),
            (3, 1000001),
        ] {
            for prefix in [
                "messages",
                "agents/BlueLake/outbox",
                "agents/RedStone/inbox",
            ] {
                let path = project_dir
                    .join(prefix)
                    .join("2026/10")
                    .join(format!("message__{id}.md"));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path,format!("---json\n{{\"id\":{id},\"from\":\"BlueLake\",\"to\":[\"RedStone\"],\"subject\":\"fixture\",\"created_ts\":{time}}}\n---\nbody\n")).unwrap();
            }
        }
        git2::Repository::init_opts(
            &root,
            git2::RepositoryInitOptions::new().initial_head("main"),
        )
        .unwrap();
        let initial_paths = walkdir::WalkDir::new(&project_dir)
            .into_iter()
            .map(Result::unwrap)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| {
                entry
                    .path()
                    .strip_prefix(&root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        mcp_agent_mail_storage::commit_paths_with_retry(
            &root,
            &mcp_agent_mail_core::Config::default(),
            "Seed retention scenario",
            &initial_paths.iter().map(String::as_str).collect::<Vec<_>>(),
        )
        .unwrap();
        let thread_digest = project_dir.join("messages/threads/digest.md");
        std::fs::create_dir_all(thread_digest.parent().unwrap()).unwrap();
        std::fs::write(&thread_digest, "old derived thread preview\n").unwrap();
        let run_dir = base.join("recovery");
        let plan = create_plan(&database, &root, "shared", 1000000, &run_dir).unwrap();
        assert_eq!(plan.expired_ids, BTreeSet::from([100, 101, 102, 103]));
        assert_eq!(plan.files.len(), 13);
        assert_eq!(
            plan.files
                .iter()
                .filter(|file| file.kind == ArchiveFileKind::ThreadDigest)
                .count(),
            1
        );
        assert_eq!(plan.message_id_floor, 103);
        let backlog = root.join(".archive_backlog");
        std::fs::create_dir(&backlog).unwrap();
        std::fs::write(backlog.join("pending.json"), b"{\"id\":100}").unwrap();
        assert!(
            apply_plan(&plan, &run_dir)
                .unwrap_err()
                .to_string()
                .contains("Archive work is pending")
        );
        assert!(
            mcp_agent_mail_db::mail_retention::read_state(&root)
                .unwrap()
                .is_none()
        );
        std::fs::rename(
            backlog.join("pending.json"),
            base.join("preserved-journal.json"),
        )
        .unwrap();
        assert_eq!(
            rows(
                &inspect_snapshot(&run_dir.join(SNAPSHOT)).unwrap(),
                "SELECT COUNT(*) FROM messages"
            )
            .unwrap(),
            vec![vec![Value::Int(7)]]
        );

        // An admission snapshot remains usable and cannot overwrite a new send.
        let changed = DbConn::open_file(database.to_str().unwrap()).unwrap();
        changed
            .execute_raw("UPDATE messages SET body_md = 'changed' WHERE id = 3")
            .unwrap();
        drop(changed);
        assert!(
            apply_plan(&plan, &run_dir)
                .unwrap_err()
                .to_string()
                .contains("changed since preview")
        );
        assert!(
            mcp_agent_mail_db::mail_retention::read_state(&root)
                .unwrap()
                .is_none()
        );
        let changed = DbConn::open_file(database.to_str().unwrap()).unwrap();
        changed
            .execute_raw("UPDATE messages SET body_md = 'keep' WHERE id = 3")
            .unwrap();
        drop(changed);
        // A fresh preview is required even when the operator restores the body,
        // because the independent lexical clock correctly recorded the writes.
        let run_dir = base.join("fresh-recovery");
        let plan = create_plan(&database, &root, "/shared", 1000000, &run_dir).unwrap();

        // Model an interruption after the first exact archive move. Startup and
        // reconstruction refuse, while applying the same manifest resumes it.
        publish_json(
            &root.join(STATE_FILE),
            &RetentionState {
                version: 1,
                complete: false,
                message_id_floor: 103,
                run_dir: run_dir.clone(),
            },
        )
        .unwrap();
        let first = &plan.files[0];
        let source = root.join(&first.relative);
        let target = run_dir.join("archive").join(&first.relative);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::rename(&source, &target).unwrap();
        assert!(mcp_agent_mail_db::mail_retention::require_complete(&root).is_err());
        let startup_config = mcp_agent_mail_core::Config {
            storage_root: root.clone(),
            ..Default::default()
        };
        assert!(
            mcp_agent_mail_server::acquire_mailbox_activity_lock(
                &startup_config,
                mcp_agent_mail_server::MailboxActivityLockMode::Exclusive
            )
            .is_err()
        );
        assert!(
            mcp_agent_mail_db::reconstruct_from_archive(&base.join("blocked.sqlite3"), &root)
                .is_err()
        );
        apply_plan(&plan, &run_dir).unwrap();
        let archive_repo = git2::Repository::open(&root).unwrap();
        let retained_head = archive_repo.head().unwrap().target().unwrap();
        apply_plan(&plan, &run_dir).unwrap();
        assert_eq!(
            archive_repo.head().unwrap().target().unwrap(),
            retained_head
        );
        let retained_tree = archive_repo.head().unwrap().peel_to_tree().unwrap();
        assert!(retained_tree.get_path(Path::new(STATE_FILE)).is_ok());
        for file in &plan.files {
            assert!(retained_tree.get_path(&file.relative).is_err());
        }
        assert_eq!(
            mcp_agent_mail_db::id_floor::max_message_id_in_archive(&root).unwrap(),
            Some(103)
        );
        let proof: serde_json::Value =
            serde_json::from_slice(&std::fs::read(run_dir.join("result.json")).unwrap()).unwrap();
        let after =
            inspect_snapshot(Path::new(proof["checked_snapshot"].as_str().unwrap())).unwrap();
        assert_eq!(
            rows(
                &after,
                "SELECT message_id FROM project_mailbox_deliveries ORDER BY message_id"
            )
            .unwrap(),
            vec![vec![Value::Int(2)], vec![Value::Int(4)]]
        );
        assert_eq!(
            rows(&after, "SELECT message_id,agent_id,read_ts,ack_ts FROM project_mailbox_receipts ORDER BY message_id")
                .unwrap(),
            vec![
                vec![Value::Int(2), Value::Int(2), Value::Int(7), Value::Int(8)],
                vec![Value::Int(4), Value::Int(3), Value::Null, Value::Null],
            ]
        );
        assert_eq!(
            rows(&after, "SELECT id FROM messages ORDER BY id").unwrap(),
            vec![
                vec![Value::Int(2)],
                vec![Value::Int(3)],
                vec![Value::Int(4)]
            ]
        );
        assert_eq!(
            rows(
                &after,
                "SELECT id,name,display_name FROM agents ORDER BY id"
            )
            .unwrap(),
            rows(
                &inspect_snapshot(&run_dir.join(SNAPSHOT)).unwrap(),
                "SELECT id,name,display_name FROM agents ORDER BY id"
            )
            .unwrap()
        );
        assert_eq!(
            rows(&after, "SELECT id FROM agent_links").unwrap(),
            vec![vec![Value::Int(19)]]
        );
        assert_eq!(
            rows(&after, "SELECT id FROM file_reservations").unwrap(),
            vec![vec![Value::Int(23)]]
        );
        assert_eq!(
            rows(
                &after,
                "SELECT message_id,read_ts,ack_ts FROM message_recipients WHERE message_id=2"
            )
            .unwrap(),
            vec![vec![Value::Int(2), Value::Int(7), Value::Int(8)]]
        );
        assert_eq!(rows(&after,"SELECT total_count,unread_count,ack_pending_count FROM inbox_stats WHERE agent_id=2").unwrap(),vec![vec![Value::Int(1),Value::Int(0),Value::Int(0)]]);
        for file in &plan.files {
            assert!(!root.join(&file.relative).exists());
            assert_eq!(
                digest(&run_dir.join("archive").join(&file.relative)).unwrap(),
                file.sha256
            );
        }
        let rebuilt = base.join("rebuilt.sqlite3");
        let stats = mcp_agent_mail_db::reconstruct_from_archive(&rebuilt, &root).unwrap();
        assert_eq!(stats.messages, 2);
        let rebuilt = DbConn::open_file(rebuilt.to_str().unwrap()).unwrap();
        assert_eq!(
            number(
                &rebuilt
                    .query_sync(
                        "SELECT seq AS id FROM sqlite_sequence WHERE name='messages'",
                        &[]
                    )
                    .unwrap()[0],
                "id"
            )
            .unwrap(),
            103
        );
        let sender = DbConn::open_file(database.to_str().unwrap()).unwrap();
        sender.execute_raw("INSERT INTO messages(project_id,sender_id,subject,body_md,created_ts) VALUES (1,1,'new send','new',1000002)").unwrap();
        assert_eq!(
            number(
                &sender
                    .query_sync("SELECT MAX(id) AS id FROM messages", &[])
                    .unwrap()[0],
                "id"
            )
            .unwrap(),
            104
        );
    }
}
