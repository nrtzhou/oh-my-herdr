//! The agent's own turn-end report. `turn-ended` is run by the hook the board passes at launch, inside the agent's
//! own turn: it only records the event and starts the finish runner, exits 0, prints nothing, and never waits more
//! than two seconds. Every judgement is made by the finish runner (finish/). A plan writer's hook runs inside its
//! read-only mount: its runner is started by the keeper instead, outside (runner/outside.rs).
use crate::agents::{helpers, installed, kinds::kind, read_turn::Turn};
use crate::clock::now;
use crate::json::*;
use crate::runner::spawn_runner;
use crate::store::files::{data_dir, marker_alive};
use crate::store::marks::{pending_events, token_ok};
use crate::store::repos::cwd_of;
use crate::store::tasks::{get, one_line, update_wait, Change, Task};
use std::io::Read;
use std::path::Path;

fn log(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(data_dir().join("hook.log")) {
        let _ = writeln!(f, "{} turn-ended: {msg}", chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
    }
}
/// The hook's input: Claude and pi send JSON on stdin, Codex appends it as the last argument.
fn input(extra: &[String]) -> Value {
    for a in extra { if a.starts_with('{') { if let Ok(v) = serde_json::from_str(a) { return v; } } }
    if unsafe { libc::isatty(0) } == 1 { return json!({}); }
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
    serde_json::from_str(if s.trim().is_empty() { "{}" } else { &s }).unwrap_or(json!({}))
}
pub fn turn_ended(id: &str, role: &str, token: &str, round: i64, extra: &[String]) {
    let mut t = if id.is_empty() { None } else { get(id) };
    // Started as one of several attempts and chosen since: its turns are the task's now.
    if let Some(c) = &t { if c.b("chosen") && c.b("attemptOf") { t = get(c.s("attemptOf")); } }
    let Some(t) = t else { return log(&format!("no task {id}")) };
    if !["agent", "reviewer"].contains(&role) || !token_ok(t.s("id"), role, token, round) { return log(&format!("refused for {id} ({role}): wrong token")); }
    let j = input(extra);
    // A task sub-agent went to work (the omp bridge's `before_subagent_spawn`): its name on the task for the board to
    // show, and no turn of its own: a helper's turns are no turns of the task's (agents/helpers.rs).
    if j.at("subagent").is_object() { return helper_started(&t, &j); }
    // The session is about to settle (the omp bridge's `session_stop`): the board may ask for one turn more. Its
    // answer goes out on the hook's own stdout, at once.
    if j.b("session_stop") {
        let a = wrap_up(&t, role);
        if a.b("continue") { let _ = update_wait(t.s("id"), 2000, |t| { t.put("wrapUp", t.i("wrapUp") + 1); Ok(Change::Write) }); }
        println!("{}", crate::json::text(&a));
        return;
    }
    // The omp bridge's other reports, each no turn of its own: a task call's own result (its name and what it said),
    // an automatic compaction, an automatic retry. One line each for the chat, kept until this turn ends.
    if j.at("task_result").is_object() || j.b("compaction") || j.b("retry_start") { return noted(&t, &j); }
    // The omp bridge's `agent_end` runs in a task sub-agent's session too, reporting the sub-agent's own conversation
    // (its session line names the parent that started it, agents/helpers.rs). A helper's turns are no turns of the
    // task's: no event of its own (MOM-18: a scout's end finished the task while its agent was still at work).
    if !helpers::omp_parent(Path::new(&j.t("transcript_path"))).is_empty() {
        return log(&format!("a sub-agent's turn end went unnoted for {id}: no turn of the task's"));
    }
    // Codex also reports turns of side conversations: a real first prompt starts with the task line, so `side` marks
    // every other report for the runner to judge. And the turns of the agent's own helpers (agents/helpers.rs), which
    // carry its first prompt along: their conversation names the thread that started them.
    let first = j.arr("input-messages").first().map(text).unwrap_or_default();
    let me = if t.b("as") { t.t("as") } else { t.t("id") };
    let session = if j.b("session_id") { j.t("session_id") } else { j.t("thread-id") };
    let helper = j.b("thread-id") && helpers::is_helper("codex", &session);
    let mut ev = json!({ "at": now(), "role": role, "round": round, "session": session, "file": j.t("transcript_path"), "error": j.t("error"),
        "side": helper || (j.b("thread-id") && !first.trim_start().starts_with(&format!("Work board task {me},"))) });
    if helper { ev["helper"] = json!(true); }
    let mut start = false;
    let r = update_wait(t.s("id"), 2000, |t| {
        let seq = t.i("eventSeq") + 1;
        t.put("eventSeq", seq);
        let mut events = pending_events(t);
        let mut e = ev.as_object().cloned().unwrap_or_default(); e.put("n", seq);
        events.push(Value::Object(e));
        t.put("events", events);
        t.del("notes"); // this turn's end takes away the lines the bridge left standing under its words (noted)
        start = !marker_alive(t.at("runner"));
        Ok(Change::Write)
    });
    match r { Ok(_) => { if start { spawn_runner("finish", t.s("id"), &[]); } } Err(e) => log(&if e.kind == crate::err::Kind::Busy { format!("gave up on {id}: the task was busy") } else { e.msg }) }
}
/// A task sub-agent its agent started (the bridge's `before_subagent_spawn`): its name and model on the task, for the
/// board to say whose work holds the turn up. No event of its own: a helper's turns are no turns of the task's.
fn helper_started(t: &Task, j: &Value) {
    let s = j.at("subagent");
    let at = now();
    let r = update_wait(t.s("id"), 2000, |t| {
        let mut hs: Vec<Value> = t.arr("helpers").iter().filter(|h| h.i("at") >= t.i("startedAgentAt")).cloned().collect();
        hs.push(json!({ "agent": s.t("agent"), "model": s.t("model"), "at": at }));
        while hs.len() > 8 { hs.remove(0); }
        t.put("helpers", hs);
        Ok(Change::Write)
    });
    if let Err(e) = r { log(&format!("helper of {id} went unnoted: {}", e.msg, id = t.s("id"))); }
}
/// One greyed line for the chat from the omp bridge's report that is no turn of its own (a task call's own result, an
/// automatic compaction, an automatic retry): kept on the task until this turn ends (turn_ended's event). A report of
/// the same kind stands again in its place.
fn noted(t: &Task, j: &Value) {
    let (kind, text) = if j.at("task_result").is_object() {
        // A background spawn's receipt (`Spawned agent …`, `async.state` `running`, no results) is not a helper back:
        // only a settled call's own results are (tools/task.md `Outputs`); a mixed call is left to its async results,
        // which are the conversation's own lines (talk.rs).
        let r = j.at("task_result");
        if r.i("results") <= 0 || r.t("state") == "running" { return; }
        let (a, n) = (one_line(&r.t("agent"), 60), one_line(&r.t("output"), 120));
        let back = if a.is_empty() { "its helper is back".to_string() } else { format!("its helper {a} is back") };
        (format!("helper {a}"), if n.is_empty() { back } else { format!("{back}: {n}") })
    } else if j.b("compaction") {
        ("compaction".to_string(), if j.t("compaction") == "start" { "Compacting: the conversation is being shortened." } else { "Compacted: the conversation was shortened." }.to_string())
    } else {
        ("retry".to_string(), "Retrying after a provider error.".to_string())
    };
    let at = now();
    let r = update_wait(t.s("id"), 2000, |t| {
        let mut ns: Vec<Value> = t.arr("notes").iter().filter(|n| n.t("kind") != kind).cloned().collect();
        ns.push(json!({ "kind": kind, "text": text, "at": at }));
        while ns.len() > 8 { ns.remove(0); }
        t.put("notes", ns);
        Ok(Change::Write)
    });
    if let Err(e) = r { log(&format!("note of {} went unnoted: {}", t.s("id"), e.msg)); }
}
/// The words the board gives an agent it asks to carry on: at a session's stop, and when you send the failure back.
pub const CARRY_ON: &str = "You stopped before the work was finished. Continue the task where you left off.";
/// The one turn more the board asks for at a session's stop (the omp bridge's `session_stop`): a turn that stopped
/// with nothing saying it finished and left no closing note — the finish line's "Stopped without a summary", asked
/// for before it is held for you. Once for a task: a second unfinished stop stands for you to judge. Never for a turn
/// that waits for you (its report or its question), went wrong, or was stopped by you.
pub fn wrap_up(t: &Task, role: &str) -> Value {
    if role != "agent" || t.i("wrapUp") >= 1 || crate::chat_only::is_chat(t) { return json!({}); }
    if !t.at("expectedWake").is_null() || !t.s("kind").is_empty() || t.b("selfBlocked") || !crate::store::inbox::pending_text(&t.s("id")).is_empty() { return json!({}); }
    if crate::flow::task::note_stat(t).map(|(_, n)| !n.trim().is_empty()).unwrap_or(false) { return json!({}); }
    let w = why_stopped(t, role, None);
    let over = w.turn.as_ref().map(|x| x.finished || !x.error.is_empty() || x.interrupted).unwrap_or(true);
    if over { return json!({}); }
    json!({ "continue": true, "additionalContext": CARRY_ON })
}
/// Why did this pane stop: { when, why, turn, hooked, kind, version, can_read }. The one seam for "when and why", fed
/// by the hook's event and the saved conversation.
pub struct Stopped { pub when: i64, pub why: String, pub turn: Option<Turn>, pub hooked: bool, pub kind: String, pub version: String, pub can_read: bool }
pub fn why_stopped(t: &Task, role: &str, event: Option<&Value>) -> Stopped {
    let k = if role == "reviewer" { t.at("link").at("reviewer").t("kind") } else { t.t("agent") };
    let ev_session = event.map(|e| e.t("session")).filter(|s| !s.is_empty());
    let session = ev_session.unwrap_or_else(|| if role == "reviewer" { t.t("reviewSession") } else { t.t("session") });
    let ev_file = event.map(|e| e.t("file")).filter(|s| !s.is_empty());
    let file = ev_file.or_else(|| (role == "agent" && t.b("transcript")).then(|| t.t("transcript")));
    let turn = installed::read_turn_of(&k, file.as_deref().map(Path::new), &session, &cwd_of(t), false);
    let why = match &turn {
        Some(x) if !x.error.is_empty() => "error".to_string(),
        Some(x) if x.interrupted => "interrupted".into(),
        Some(x) if x.finished => if x.stop.is_empty() { "stop".into() } else { x.stop.clone() },
        _ => if event.is_some() { "stop".into() } else { String::new() },
    };
    let when = event.map(|e| e.i("at")).filter(|w| *w != 0).or_else(|| turn.as_ref().map(|x| x.pos).filter(|p| *p != 0)).unwrap_or(0);
    Stopped { when, why, hooked: event.is_some(), version: installed::version_of(&k), can_read: kind(&k).map(|a| a.reads_turn).unwrap_or(false), kind: k, turn }
}
