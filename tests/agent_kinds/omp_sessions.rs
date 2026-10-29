//! omp's saved conversations are looked for only in the folder omp keeps for the agent's working folder, never in
//! every folder of a long history: a task's transcript and the newest conversation of a folder are both found there.
use crate::common;
use common::*;
use serde_json::json;
use std::path::Path;
use sw::agents::adapter::{latest_session, transcript};

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
