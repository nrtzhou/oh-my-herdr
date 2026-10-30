//! omp's saved conversations, read back: they are looked for only in the folder omp keeps for the agent's working
//! folder, never in every folder of a long history (a task's transcript and the newest conversation of a folder are
//! both found there), and a conversation's prompts are its own words alone.
use crate::common;
use common::*;
use serde_json::json;
use std::path::Path;
use sw::agents::adapter::{find_session, find_session_but, latest_session, transcript};
use sw::agents::prompts::prompts_of;

#[test]
fn an_omp_conversation_is_found_in_its_own_folders_place_and_not_in_another_folders() {
    let _w = world_with(Opts::default());
    let sessions = Path::new(&std::env::var("OMP_CODING_AGENT_DIR").unwrap()).join("sessions");
    let (mine, other) = (Path::new("/work/app"), Path::new("/work/other"));
    // omp writes a title slot first and the session's own entry second.
    let save = |cwd: &Path, id: &str| { let f = sessions.join(omp_folder(cwd)).join(format!("2026-10-28T00-00-00-000Z_{id}.jsonl")); std::fs::create_dir_all(f.parent().unwrap()).unwrap(); std::fs::write(&f, format!("{}\n{}\n", json!({ "type": "title", "v": 1, "title": "T", "pad": "" }), json!({ "type": "session", "version": 3, "id": id, "cwd": cwd }))).unwrap(); f };
    let f = save(mine, "omp-mine");
    save(other, "omp-theirs");
    assert_eq!(transcript("omp", "omp-mine", mine), Some(f));
    assert_eq!(transcript("omp", "omp-theirs", mine), None, "another folder's is not looked through");
    assert_eq!(latest_session("omp", mine, 0, None).as_deref(), Some("omp-mine"));
}

#[test]
fn without_a_hook_an_omp_task_conversation_is_found_by_its_prompts_and_never_a_sub_agents() {
    let _w = world_with(Opts::default());
    let sessions = Path::new(&std::env::var("OMP_CODING_AGENT_DIR").unwrap()).join("sessions");
    let cwd = Path::new("/work/app");
    let since = sw::clock::now();
    let (at, old) = (sw::clock::iso(since), sw::clock::iso(since - 3600000));
    let save = |f: &Path, id: &str, parent: Option<&str>, when: &str, words: &str| {
        std::fs::create_dir_all(f.parent().unwrap()).unwrap();
        let mut meta = json!({ "type": "session", "version": 3, "id": id, "cwd": cwd.display().to_string(), "timestamp": when });
        if let Some(p) = parent { meta["parentSession"] = json!(p); }
        let said = json!({ "type": "message", "id": "u1", "timestamp": when, "message": { "role": "user", "content": [{ "type": "text", "text": words }] } });
        std::fs::write(f, format!("{}\n{}\n{}\n", json!({ "type": "title", "v": 1, "title": "T", "pad": "" }), meta, said)).unwrap();
    };
    let folder = sessions.join(omp_folder(cwd));
    let mine = folder.join("2026-10-30T00-00-00-000Z_omp-task.jsonl");
    let words = "Work board task proj/7, role: agent. Do it.";
    save(&mine, "omp-task", None, &at, words);
    save(&mine.with_extension("").join("Sub.jsonl"), "omp-sub", Some(&mine.display().to_string()), &at, words); // a sub-agent's, with the same words along
    save(&folder.join("2026-10-30T00-00-00-000Z_omp-other.jsonl"), "omp-other", None, &at, "Work board task proj/9, role: agent.");
    save(&folder.join("2026-10-30T00-00-00-000Z_omp-old.jsonl"), "omp-old", None, &old, words); // begun before the program now running
    assert_eq!(find_session("omp", cwd, since, "Work board task proj/7").as_deref(), Some("omp-task"));
    assert_eq!(find_session("omp", cwd, since, "no such task"), None);
    assert_eq!(find_session_but("omp", cwd, since, "Work board task proj/7", "omp-task"), None, "one already known to be another's is left out");
}

#[test]
fn an_omp_conversations_prompts_are_its_own_words_in_order() {
    let _w = world_with(Opts::default());
    let f = Path::new(&std::env::var("OMP_CODING_AGENT_DIR").unwrap()).join("sessions").join("one").join("2026_s.jsonl");
    std::fs::create_dir_all(f.parent().unwrap()).unwrap();
    let lines = [
        json!({ "type": "title", "v": 1, "title": "", "pad": "" }),
        json!({ "type": "session", "version": 3, "id": "s", "cwd": "/w", "timestamp": "2026-10-30T00:00:00.000Z" }),
        json!({ "type": "message", "id": "u1", "timestamp": "2026-10-30T00:00:01.000Z", "message": { "role": "user", "content": [{ "type": "text", "text": "Work board task proj/7, role: agent." }] } }),
        json!({ "type": "message", "id": "a1", "timestamp": "2026-10-30T00:00:02.000Z", "message": { "role": "assistant", "content": [{ "type": "text", "text": "On it." }], "stopReason": "stop" } }),
        json!({ "type": "message", "id": "t1", "timestamp": "2026-10-30T00:00:03.000Z", "message": { "role": "toolResult", "content": [{ "type": "text", "text": "3 files" }] } }),
        json!({ "type": "custom_message", "customType": "session-agent-notice", "content": "A note.", "timestamp": "2026-10-30T00:00:04.000Z" }),
        json!({ "type": "message", "id": "u2", "timestamp": "2026-10-30T00:00:05.000Z", "message": { "role": "user", "content": [{ "type": "text", "text": "Now f2." }] } }),
    ];
    std::fs::write(&f, lines.iter().map(|l| format!("{l}\n")).collect::<String>()).unwrap();
    let got = prompts_of("omp", Some(&f)).unwrap();
    assert_eq!(got.iter().map(|(_, p)| p.as_str()).collect::<Vec<_>>(), ["Work board task proj/7, role: agent.", "Now f2."], "no tool result and no notice of the program's own among them");
}
