//! The omp hook bridge's reports that are no turn of their own: a task's helper back with what it said, an automatic
//! compaction, an automatic retry. One greyed line each under its agent's words in the chat, kept until its turn ends —
//! never counted as a turn of the task's.
use crate::common;
use common::*;
use serde_json::json;
use sw::json::*;

/// A task with an omp agent at work and the token its hook reports under.
fn ready() -> (World, String) {
    let w = world_with(Opts { repo: true, settings: json!({ "worker.agent": "omp", "review.when": "never" }), ..Default::default() });
    w.script(json!({ "worker": [{ "status": "working" }] }));
    let id = w.add("Count the words");
    delegate(&id);
    sw::store::marks::set_token(&id, "agent", "tok-1", 0).unwrap();
    (w, id)
}
/// What the chat says under the line that names who you talk to.
fn note(w: &World, id: &str) -> String {
    let mut b = common::board::open(&w.root, 40);
    b.s.reply = true;
    let mut row = task(id);
    row.put("snap", sw::board::data::snap(&row));
    sw::board::view::chat::chat_of(&mut b.s, &b.theme, Some(&row)).unwrap().note
}
/// The bridge's report, as the hook sends it on the shim's stdin.
fn report(id: &str, j: serde_json::Value) { sw::turn::turn_ended(id, "agent", "tok-1", 0, &[j.to_string()]); }

#[test]
fn a_background_spawn_s_receipt_is_no_helper_back_and_a_settled_call_s_own_result_is() {
    let (w, id) = ready();
    // The receipt of a background spawn (tools/task.md's background response): no result yet, so no helper is back.
    report(&id, json!({ "task_result": { "agent": "BuildAll", "output": "", "state": "running", "results": 0 } }));
    assert_eq!(note(&w, &id), "", "the receipt says no helper is back");
    assert_eq!(task(&id).i("eventSeq"), 0);
    // A settled call comes back with its own results: one line, until its turn ends.
    report(&id, json!({ "task_result": { "agent": "BuildAll", "output": "Built all three.", "state": "", "results": 1 } }));
    assert_eq!(note(&w, &id), "its helper BuildAll is back: Built all three.");
    assert_eq!(task(&id).i("eventSeq"), 0, "a helper back is no turn of the task's");
    // the real turn's end takes the line away and stands as the turn it is
    report(&id, json!({ "transcript_path": "" }));
    assert_eq!(note(&w, &id), "", "its turn's end takes the line away");
    assert_eq!(task(&id).i("eventSeq"), 1);
}

/// The whole life of a background spawn as the board sees it (tools/task.md `Outputs`): its receipt says nothing, and
/// its result comes back into the conversation itself (talk.rs's own line for an `async-result`), not as a note.
#[test]
fn a_background_spawn_s_result_comes_back_into_the_conversation_itself() {
    let (w, id) = ready();
    report(&id, json!({ "task_result": { "agent": "BuildAll", "output": "", "state": "running", "results": 0 } }));
    assert_eq!(note(&w, &id), "", "the receipt says no helper is back");
    let dir = w.base.join("omp-life");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("session.jsonl");
    std::fs::write(&file, format!("{}\n", json!({ "type": "custom_message", "customType": "async-result", "content": "<system-notice>\nBackground job bg_1 has completed.", "timestamp": 1 }))).unwrap();
    let said = sw::agents::talk::talk_of("omp", Some(&file), "", std::path::Path::new("/"), 50).unwrap().iter().map(|m| m.text.clone()).collect::<Vec<_>>();
    assert!(said.contains(&"Background job finished: its result went into the conversation.".to_string()), "{said:?}");
}

#[test]
fn a_mixed_call_s_receipt_says_nothing_while_its_background_spawns_run() {
    let (w, id) = ready();
    // Its settled spawns' results ride the receipt, but the call is still running: its async results are the
    // conversation's own lines when they come back.
    report(&id, json!({ "task_result": { "agent": "BuildAll", "output": "Built one.", "state": "running", "results": 1 } }));
    assert_eq!(note(&w, &id), "");
    assert_eq!(task(&id).i("eventSeq"), 0);
}

#[test]
fn an_automatic_compaction_is_one_line_and_never_a_turn_of_the_tasks() {
    let (w, id) = ready();
    report(&id, json!({ "compaction": "start" }));
    assert_eq!(note(&w, &id), "Compacting: the conversation is being shortened.");
    report(&id, json!({ "compaction": "end" }));
    assert_eq!(note(&w, &id), "Compacted: the conversation was shortened.");
    assert_eq!(task(&id).i("eventSeq"), 0, "the board never mistook the compaction for a turn");
}

#[test]
fn an_automatic_retry_is_one_line_and_never_a_turn_of_the_tasks() {
    let (w, id) = ready();
    report(&id, json!({ "retry_start": true }));
    assert_eq!(note(&w, &id), "Retrying after a provider error.");
    assert_eq!(task(&id).i("eventSeq"), 0);
}

/// The hook file the board writes for an omp run wires every event the bridge reports on: a dropped subscription is
/// silent, and nothing else here would notice it.
#[test]
fn the_hook_file_wires_every_event_the_bridge_reports() {
    let (w, id) = ready();
    let h = sw::agents::adapter::Hook { shim: "/bin/board".into(), id, role: "agent".into(), token: "tok-1".into(), round: 0, dir: w.base.clone() };
    let a = sw::agents::adapter::hook_args("omp", &h).unwrap().unwrap();
    let file = &a[a.iter().position(|x| x == "-e").unwrap() + 1];
    let ts = std::fs::read_to_string(file).unwrap();
    for on in ["agent_end", "before_subagent_spawn", "session_stop", "tool_result", "auto_compaction_start", "auto_compaction_end", "auto_retry_start"] {
        assert!(ts.contains(&format!("omp.on(\"{on}\"")), "{on} is not wired");
    }
}
