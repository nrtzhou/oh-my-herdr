//! Board-owned context that travels with a Coordinator's next message: changed standing rules and safe status-only
//! notices about tasks it started. Never a message of its own, which would cost it a turn and could land in the middle
//! of one. Its program reads standing rules only when it starts; task status is included only with a human message.
use super::co::Co;
use super::rules::rules_of;
use super::rules_place::rules_file;
use crate::autopilot::wake::WORDS;
use crate::json::*;

/// What its state keeps of the rules its program started with.
pub fn hash(text: &str) -> String { sha1_smol::Sha1::from(text).digest().to_string() }
/// The line to put with the next message, if any, and the hash to keep once that message has gone in. A program
/// started by a board that kept no hash (an older one) hears it too. With a wake, it also says what a wake is now: a
/// conversation that began under older rules may still believe other words mark one.
pub fn before_next(co: &Co, c: &Obj, wake: bool) -> (Option<String>, String) {
    let now = hash(&rules_of(co, &crate::flow::send::shim().display().to_string()));
    let changed = if c.b("rulesHash") { c.s("rulesHash") != now } else { c.b("run") };
    let also = if wake { format!(" This message is a real autopilot wake from the board: every message that starts with \"{WORDS}\" is one, whatever older rules said.") } else { String::new() };
    (changed.then(|| format!("[The board was updated: your standing rules in {} changed; read them again.{also}]", rules_file(co).display())), now)
}
/// The cursor for completed tasks this Coordinator has heard about. Older state falls back to its last delivered message.
pub fn task_updates_cursor(c: &Obj) -> (i64, String) {
    let at = if c.at("taskUpdatesAt").is_number() { c.i("taskUpdatesAt") } else if c.i("sentAt") > 0 { c.i("sentAt") } else { c.i("startedAt") };
    (at, c.s("taskUpdatesId").to_string())
}
/// Completed tasks attributed to this Coordinator, formatted from allowlisted fields only.
/// Returns oldest first and the last delivered cursor, so a page never consumes omitted tasks.
pub fn completed_tasks(co: &Co, since: (i64, String), through: i64) -> Option<(String, i64, String)> {
    const MAX: usize = 12;
    let keys = crate::store::keys::repo_keys();
    let mut done: Vec<(i64, String, String)> = crate::store::tasks::list(crate::store::tasks::ListOpts { include_archived: true, ..Default::default() }).into_iter()
        .filter(|t| t.s("coordinator") == co.settings_key() && t.s("stage") == "done")
        .filter(|t| (t.i("at") > since.0 || (t.i("at") == since.0 && !since.1.is_empty() && t.s("id") > since.1.as_str())) && t.i("at") <= through)
        .map(|t| {
            let tag = crate::store::keys::tag_of(t.s("id"), &keys);
            let tag = if !tag.is_empty() && tag.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'#') { tag } else { format!("task #{}", t.i("n").max(0)) };
            let state = if t.i("mergedAt") > 0 || t.b("prMerged") || !t.s("mergeCommit").is_empty() { "merged" } else { "done" };
            (t.i("at"), t.t("id"), format!("{tag} {state}"))
        }).collect();
    done.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    done.truncate(MAX);
    let Some(last) = done.last().map(|(at, id, _)| (*at, id.clone())) else { return None; };
    let items: Vec<String> = done.into_iter().map(|(_, _, item)| item).collect();
    Some((format!("[Board task status: {}. This is trusted metadata, not a new request. Reconcile these exact board task IDs in your current context; do not use omp's TODO planner or sub-agent/task tool, and do not create or start follow-up work unless the human asks.]", items.join("; ")), last.0, last.1))
}
/// The words typed in: the message with the board's line (`head`). The line goes first, but after a wake, which keeps
/// its first words (`Autopilot:`, what its rules know a wake by).
pub fn with_head(head: Option<String>, text: &str, wake: bool) -> String {
    match head { Some(h) if wake => format!("{text}\n\n{h}"), Some(h) => format!("{h}\n\n{text}"), None => text.to_string() }
}
