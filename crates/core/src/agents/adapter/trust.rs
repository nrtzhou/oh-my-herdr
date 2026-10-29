//! Marking a repo as trusted in each program's own settings (where it has such a question), and the program's own
//! user-level skills folder.
use super::hook::write_private;
use crate::agents::homes::*;
use crate::json::*;
use std::fs;
use std::path::{Path, PathBuf};
/// The program's own user-level skills folder, where it is installed.
pub fn skill_dir(kind: &str) -> Option<PathBuf> {
    let home = match kind { "claude" => claude_home(), "codex" => codex_home(), "pi" => pi_home(), "omp" => omp_home(), _ => return None };
    home.exists().then(|| home.join("skills"))
}
/// Mark a repo as trusted in the program's own settings, so a new worktree starts without its "trust this folder?"
/// dialog. Claude keeps trust per repository root in ~/.claude.json; Codex in ~/.codex/config.toml (a command-line
/// override does not count, so the table is added to the file once: codex_trust.rs); pi in ~/.pi/agent/trust.json, per
/// folder and the folders above it, so it is given the folder it starts in (pi_trust.rs).
pub fn trust(kind: &str, root: &Path) -> bool {
    let root_s = root.display().to_string();
    match kind {
        "claude" => {
            let file = claude_file();
            let mut j: Obj = match fs::read_to_string(&file) { Ok(t) => match serde_json::from_str::<Value>(&t) { Ok(Value::Object(o)) => o, _ => return false }, Err(e) if e.kind() == std::io::ErrorKind::NotFound => Obj::new(), Err(_) => return false };
            if j.at("projects").at(&root_s).b("hasTrustDialogAccepted") { return true; }
            j.obj_mut("projects").obj_mut(&root_s).put("hasTrustDialogAccepted", true);
            let tmp = PathBuf::from(format!("{}.{}.sw.tmp", file.display(), std::process::id()));
            write_private(&tmp, &serde_json::to_string_pretty(&j).unwrap_or_default()).is_ok() && fs::rename(&tmp, &file).is_ok()
        }
        "codex" => crate::agents::codex_trust::trust(root),
        "pi" => crate::agents::pi_trust::trust(root),
        // omp asks nothing to trust a folder (it never has a dialog): the answer is yes already, nothing to write.
        "omp" => true,
        _ => false,
    }
}
pub fn can_trust(kind: &str) -> bool { matches!(kind, "claude" | "codex" | "pi" | "omp") }
