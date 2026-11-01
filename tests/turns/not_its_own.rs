//! A turn end that is not the agent's own (MOM-18, 2026-11-01): a task sub-agent's turn end is no turn of the task's,
//! and no end at all counts while the agent's own conversation is still in a turn. Both let a scout's closing report
//! finish the task while its agent was still at work.
use crate::common;
use common::*;
use serde_json::json;
use std::path::Path;
use sw::json::*;

/// One omp conversation: its session line (a task sub-agent's names the conversation that started it), an open prompt,
/// and the last message. `stop` ends the turn; `toolUse` leaves it open, as the agent's own is while it works.
fn convo(file: &Path, parent: &str, at: i64, id: &str, said: &str, stop: &str) {
    let iso = sw::clock::iso(at);
    let mut session = json!({ "type": "session", "version": 3, "id": id, "cwd": "/w", "timestamp": iso });
    if !parent.is_empty() { session["parentSession"] = json!(parent); }
    let lines = [
        json!({ "type": "title", "v": 1, "title": "", "pad": "" }),
        session,
        json!({ "type": "message", "id": format!("u-{id}"), "timestamp": iso, "message": { "role": "user", "content": [{ "type": "text", "text": "Work." }] } }),
        json!({ "type": "message", "id": format!("a-{id}"), "timestamp": iso, "message": { "role": "assistant", "content": [{ "type": "text", "text": said }], "stopReason": stop } }),
    ];
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(file, lines.iter().map(|l| format!("{l}\n")).collect::<String>()).unwrap();
}
/// A task whose omp agent is at work (its own turn open), on the board's `session` and `transcript`.
fn working_omp() -> (World, String, std::path::PathBuf, i64) {
    let w = world_with(Opts { repo: true, settings: json!({ "worker.agent": "omp", "review.when": "never" }), ..Default::default() });
    w.script(json!({ "worker": [{ "status": "working" }] }));
    let id = w.add("Count the words");
    delegate(&id);
    assert_eq!(settle(&id).s("stage"), "working");
    let at = sw::clock::now();
    let dir = w.base.join("omp convos");
    let main = dir.join("main.jsonl");
    convo(&main, "", at, "main", "Working on it.", "toolUse");
    sw::store::tasks::patch(&id, |t| { t.put("session", main.display().to_string()); t.put("transcript", main.display().to_string()); t.put("startedAgentAt", at); }).unwrap();
    sw::store::marks::set_token(&id, "agent", "tok-1", 0).unwrap();
    (w, id, main, at)
}

#[test]
fn a_task_sub_agent_s_turn_end_is_no_turn_of_the_task_s() {
    let (_w, id, main, at) = working_omp();
    // the sub-agent's own conversation, beside its parent's: its turn is over, its report said
    let sub = main.parent().unwrap().join("sub.jsonl");
    convo(&sub, &main.display().to_string(), at + 1000, "sub", "Nine.", "stop");
    // the bridge's agent_end report, as the hook sends it: the sub-agent's own session file
    sw::turn::turn_ended(&id, "agent", "tok-1", 0, &[json!({ "transcript_path": sub.display().to_string() }).to_string()]);
    let t = settle(&id);
    assert_eq!(t.i("eventSeq"), 0, "a helper's turn end is no event of the task's");
    assert_eq!(t.s("stage"), "working", "{}", t.s("msg"));
    assert!(t.s("said").is_empty(), "{}", t.s("said"));
}

#[test]
fn no_finish_while_the_agent_s_own_conversation_is_still_in_a_turn() {
    let (_w, id, main, at) = working_omp();
    // another conversation of the same program ends a turn (any end that is not its own: a helper's, a stale report)
    let foreign = main.parent().unwrap().join("foreign.jsonl");
    convo(&foreign, "", at + 1000, "foreign", "Foreign report.", "stop");
    sw::turn::turn_ended(&id, "agent", "tok-1", 0, &[json!({ "transcript_path": foreign.display().to_string() }).to_string()]);
    let t = settle(&id);
    assert_eq!(t.s("stage"), "working", "{}", t.s("msg"));
    assert!(t.s("said").is_empty(), "{}", t.s("said"));
}
