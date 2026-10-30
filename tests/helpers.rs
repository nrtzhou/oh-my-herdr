//! Helpers an agent runs inside its own work (Codex's sub-agents, Claude Code's background agents): a helper's turn
//! ending is not the review's or the task's end, a turn that ends while a helper is still at work is waited past, and
//! a reviewer that really ended without a verdict is started once more by itself. Seen on HP-1 (2026-10-15): a Codex
//! reviewer's helper `/root/check_plans` finished, and the board failed the review for want of a verdict.
use crate::common;
use common::*;
use serde_json::{json, Value};
use sw::json::*;
use sw::store::reviews::{review_history, review_round};

fn work() -> Value { json!({ "files": { "f.txt": "v1\n" }, "commit": "v1", "note": "Did v1." }) }
fn verdict() -> Value { json!({ "meets": true, "findings": [], "text": "Looks right." }) }
fn reviewer_starts(w: &World) -> usize { calls(w).iter().filter(|c| regex::Regex::new(r"^agent start \S+-r ").unwrap().is_match(c)).count() }
fn passed(t: &Obj) { assert_eq!(t.s("stage"), "approval", "{} {}", t.s("msg"), t.at("failure")); assert!(review_history(t).iter().all(|e| !e.b("error")), "{:?}", review_history(t)); }

#[test]
fn a_codex_reviewers_helper_finishing_its_turn_is_not_the_review_ending() {
    let w = cut_world("30", json!({ "reviewer.agent": "codex" }));
    // The helper's report carries the reviewer's own first prompt, as Codex sends it: only its conversation tells.
    let back = json!({ "verdict": verdict(), "ends": "codex-finished-made", "hook": true });
    w.script(json!({ "worker": [work()], "reviewer": [{ "helper": true, "status": "working", "then": back }] }));
    let id = w.add("Helped review");
    delegate(&id);
    let t = settle(&id);
    passed(&t); assert_eq!(reviewer_starts(&w), 1);
}

#[test]
fn a_codex_reviewer_whose_turn_ends_while_its_helper_is_at_work_is_waited_for() {
    let w = cut_world("30", json!({ "reviewer.agent": "codex" }));
    let back = json!({ "verdict": verdict(), "ends": "codex-finished-made", "hook": true });
    w.script(json!({ "worker": [work()], "reviewer": [{ "helper": "running", "ends": "codex-helper-running-made", "hook": true, "then": back }] }));
    let id = w.add("Waited for its helper");
    delegate(&id);
    let t = settle(&id);
    passed(&t); assert_eq!(reviewer_starts(&w), 1);
}

#[test]
fn a_claude_reviewer_whose_background_agent_is_still_at_work_is_waited_for() {
    let w = cut_world("30", json!({ "reviewer.agent": "claude" }));
    let back = json!({ "verdict": verdict(), "ends": "claude-helper-back-made", "hook": true });
    w.script(json!({ "worker": [work()], "reviewer": [{ "ends": "claude-helper-running-made", "hook": true, "then": back }] }));
    let id = w.add("Background helper");
    delegate(&id);
    let t = settle(&id);
    passed(&t); assert_eq!(reviewer_starts(&w), 1);
}

#[test]
fn a_reviewer_that_ended_without_a_verdict_is_started_once_more_by_itself_as_the_same_round() {
    let w = cut_world("30", json!({}));
    w.script(json!({ "worker": [work()], "reviewer": [{ "ends": "codex-finished-made" }, { "verdict": verdict() }] }));
    let id = w.add("Second look");
    delegate(&id);
    let t = settle(&id);
    passed(&t); assert_eq!(reviewer_starts(&w), 2);
    assert_eq!(review_round(&t), 1);
}

#[test]
fn a_codex_workers_helper_finishing_does_not_finish_the_task() {
    let w = cut_world("30", json!({ "worker.agent": "codex" }));
    let mut back = work(); back["ends"] = json!("codex-finished-made"); back["hook"] = json!(true);
    w.script(json!({ "worker": [{ "helper": true, "status": "working", "after": 0, "then": back }] }));
    let id = w.add("Helped work");
    delegate(&id);
    let t = settle(&id);
    assert_eq!((t.s("stage"), reviewer_starts(&w)), ("working", 0), "{}", t.s("msg"));
    let looks = calls(&w).len();
    assert_eq!(settle(&id).s("stage"), "working"); // looked at again, it still waits for its helper
    assert!(calls(&w).len() - looks < 20, "{:?}", &calls(&w)[looks..]);
    w.go_on(&t);
    passed(&settle(&id));
}

#[test]
fn a_claude_worker_whose_background_agent_is_still_at_work_is_not_finished_until_it_is_back() {
    let w = cut_world("30", json!({ "worker.agent": "claude" }));
    let mut back = work(); back["ends"] = json!("claude-helper-back-made"); back["hook"] = json!(true);
    w.script(json!({ "worker": [{ "ends": "claude-helper-running-made", "hook": true, "after": 0, "then": back }] }));
    let id = w.add("Background work");
    delegate(&id);
    let t = settle(&id);
    assert_eq!((t.s("stage"), reviewer_starts(&w)), ("working", 0), "{}", t.s("msg"));
    let looks = calls(&w).len();
    assert_eq!(settle(&id).s("stage"), "working"); // looked at again, it still waits for its helper
    assert!(calls(&w).len() - looks < 20, "{:?}", &calls(&w)[looks..]);
    w.go_on(&t);
    passed(&settle(&id));
}

#[test]
fn a_quiet_helper_is_at_work_however_long_and_one_started_before_its_program_is_gone() {
    let _w = world();
    let started = sw::clock::now();
    let at = sw::clock::iso(started + 1000);
    let write = |file: &std::path::Path, lines: &[Value]| { std::fs::create_dir_all(file.parent().unwrap()).unwrap(); std::fs::write(file, lines.iter().map(|l| format!("{l}\n")).collect::<String>()).unwrap(); };
    // Codex: the agent's own conversation says it started a helper, whose own turn is open (a long build, no word).
    let sessions = std::path::PathBuf::from(std::env::var("CODEX_HOME").unwrap()).join("sessions").join("2026");
    let root = sessions.join("rollout-2026-root-quiet.jsonl");
    write(&root, &[json!({ "timestamp": at, "type": "event_msg", "payload": { "type": "item_completed", "item": { "type": "SubAgentActivity", "kind": "started", "agent_thread_id": "helper-quiet", "agent_path": "/root/build" } } })]);
    write(&sessions.join("rollout-2026-helper-quiet.jsonl"), &[
        json!({ "timestamp": at, "type": "session_meta", "payload": { "id": "helper-quiet", "parent_thread_id": "root-quiet", "source": { "subagent": {} } } }),
        json!({ "timestamp": at, "type": "event_msg", "payload": { "type": "task_started" } }),
        json!({ "timestamp": at, "type": "response_item", "payload": { "type": "function_call", "name": "exec_command" } }),
    ]);
    // Claude Code: a background agent launched, its notification not back.
    let claude = std::path::PathBuf::from(std::env::var("CLAUDE_CONFIG_DIR").unwrap()).join("projects").join("x").join("s.jsonl");
    write(&claude, &[json!({ "timestamp": at, "type": "user", "message": { "role": "user", "content": [{ "type": "tool_result", "content": "Async agent launched successfully.\nagentId: aquiet01 (internal ID)" }] } })]);
    // omp: a task sub-agent's conversation beside the agent's own, its turn open (a long build). A sub-agent that
    // finished, an advisor's review and one belonging to another conversation are no helpers of this one.
    let omp = std::path::PathBuf::from(std::env::var("OMP_CODING_AGENT_DIR").unwrap()).join("sessions").join("one").join("2026-10-30T00-00-00-000Z_root-quiet.jsonl");
    write(&omp, &[
        json!({ "type": "title", "v": 1, "title": "", "pad": "" }),
        json!({ "type": "session", "version": 3, "id": "root-quiet", "cwd": "/work/x", "timestamp": at }),
    ]);
    let sub = |name: &str| omp.with_extension("").join(format!("{name}.jsonl"));
    let open = |id: &str, parent: &str| vec![
        json!({ "type": "title", "v": 1, "title": "", "pad": "" }),
        json!({ "type": "session", "version": 3, "id": id, "cwd": "/work/x", "parentSession": parent, "timestamp": at }),
        json!({ "type": "message", "id": "u1", "timestamp": at, "message": { "role": "user", "content": [{ "type": "text", "text": "Build it." }] } }),
    ];
    let root_s = omp.display().to_string();
    write(&sub("BuildAll"), &open("sub-build", &root_s));
    write(&sub("Orphan"), &open("sub-orphan", "/elsewhere/other.jsonl"));
    write(&sub("__advisor"), &open("advisor", &root_s));
    let mut done = open("sub-done", &root_s);
    done.push(json!({ "type": "message", "id": "a1", "timestamp": at, "message": { "role": "assistant", "content": [{ "type": "text", "text": "Five." }], "stopReason": "stop" } }));
    write(&sub("DoneOne"), &done);
    sw::clock::advance(3 * 3600000);
    assert_eq!(sw::agents::helpers::running("codex", &root, started), 1);
    assert_eq!(sw::agents::helpers::running("claude", &claude, started), 1);
    assert_eq!(sw::agents::helpers::running("omp", &omp, started), 1, "the open one only: not the finished one, the advisor or another conversation's");
    // The program was started again since (a resumed conversation): helpers of the one before are gone with it.
    let again = sw::clock::now();
    assert_eq!(sw::agents::helpers::running("codex", &root, again), 0);
    assert_eq!(sw::agents::helpers::running("claude", &claude, again), 0);
    assert_eq!(sw::agents::helpers::running("omp", &omp, again), 0);
    assert!(sw::agents::helpers::is_helper("codex", "helper-quiet") && !sw::agents::helpers::is_helper("codex", "root-quiet"));
}
