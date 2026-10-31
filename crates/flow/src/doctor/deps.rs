//! One area of `omh doctor`: a repo whose code needs its dependencies installed and has no `workspace.setup`, so every
//! task in it improvises them (an npx of the wrong version, a symlink that two agents fight over).
use super::Found;
use crate::store::repos::known_repos;
use crate::store::settings::lookup::{lookup, Scope};
use crate::store::settings::places::yours;
use crate::store::settings::values::command_of;
use crate::json::text;
use std::path::{Path, PathBuf};

/// The lockfiles that say a repo's code needs an install step before it runs.
const LOCKS: [&str; 3] = ["pnpm-lock.yaml", "package-lock.json", "yarn.lock"];

/// Does this folder, or one folder under it, hold a lockfile?
pub fn locked(root: &Path) -> bool {
    let has = |d: &Path| LOCKS.iter().any(|l| d.join(l).is_file());
    has(root) || std::fs::read_dir(root).map(|rd| rd.flatten().any(|e| e.path().is_dir() && has(&e.path()))).unwrap_or(false)
}
/// Each known repo: its name, the `workspace.setup` it resolves to, and whether its code holds a lockfile.
pub fn ask() -> Vec<(String, String, bool)> {
    known_repos().iter().map(|(name, v)| {
        let root = PathBuf::from(text(v));
        let setup = lookup("workspace.setup", &Scope { repo: name.clone(), ..Default::default() }, &yours()).map(|f| command_of(&f.value)).unwrap_or_default();
        (name.clone(), setup, locked(&root))
    }).collect()
}
/// A repo that needs an install step and has none is a problem; the line says what to do.
pub fn check(asked: &[(String, String, bool)]) -> Vec<Found> {
    let bad = asked.iter().filter(|(_, setup, locked)| *locked && setup.is_empty()).collect::<Vec<_>>();
    if bad.is_empty() { return vec![Found::ok("Every repo that needs its dependencies installed has a workspace.setup")]; }
    bad.iter().map(|(name, _, _)| Found::bad(format!("{name}: its code holds a lockfile but workspace.setup is none, so every task improvises its dependencies"),
        format!("Set workspace.setup for {name} (an install that follows its lockfile, as `pnpm install --frozen-lockfile`): it runs in the fresh worktree before the agent starts"))).collect()
}
