//! How a program is started and reopened: the conversation id the board gives it (where it takes one), the arguments
//! of a new conversation and of a reopened one, the standing rules kept for the session, and how a model and an
//! effort reach the program.
use crate::agents::kinds::fill;
use crate::err::Res;
use crate::util::uuid;
use std::fs;
use std::path::Path;
/// A conversation id of the board's own choosing, where the program takes one (kinds.rs `session`).
pub fn new_session(kind: &str) -> Option<String> {
    match kind { "pi" => Some(format!("sw-{}", &uuid()[..13])), k => crate::agents::kinds::kind(k).filter(|a| !a.session.is_empty()).map(|_| uuid()) }
}
/// The arguments that start a new conversation: the id the board chose for it, and the program's own "a new one".
pub fn session_args(kind: &str, id: &str) -> Vec<String> { crate::agents::kinds::kind(kind).map(|a| if id.is_empty() { vec![] } else { fill(a.session, id) }).unwrap_or_default() }
pub fn fresh_args(kind: &str) -> Vec<String> { crate::agents::kinds::kind(kind).map(|a| fill(a.fresh, "")).unwrap_or_default() }
/// Launch arguments that keep the rules in force for the whole session. herdr refuses launch arguments that hold line
/// breaks, so a program reads them from a file (kinds.rs `standing`); Codex takes them as a setting, and is pointed at
/// a file when they are long.
pub fn standing(kind: &str, text: &str, dir: &Path) -> Res<Vec<String>> {
    // Inline when short; else a file it is told to read: a long start command is cut off by the terminal (hand_limits.rs).
    if kind == "codex" {
        let inline = serde_json::to_string(text)?;
        if inline.len() <= crate::agents::hand_limits::ARG_MAX { return Ok(vec!["-c".into(), format!("developer_instructions={inline}")]); }
        let f = dir.join("rules.md"); fs::write(&f, text)?;
        return Ok(vec!["-c".into(), format!("developer_instructions={}", serde_json::to_string(&crate::agents::hand_limits::rules_pointer(&f))?)]);
    }
    let Some(a) = crate::agents::kinds::kind(kind).filter(|a| !a.standing.is_empty()) else { return Ok(vec![]) };
    let f = dir.join("rules.md");
    fs::write(&f, text)?;
    Ok(fill(a.standing, &f.display().to_string()))
}
pub fn has_standing(kind: &str) -> bool { kind == "codex" || crate::agents::kinds::kind(kind).map(|a| !a.standing.is_empty()).unwrap_or(false) }
pub fn resume_args(kind: &str, id: &str) -> Vec<String> { crate::agents::kinds::kind(kind).map(|a| fill(a.resume, id)).unwrap_or_default() }
/// pi takes a provider apart from its model (`pi/anthropic/claude-opus-5-5`).
pub fn model_args(kind: &str, m: &str) -> Vec<String> {
    match kind {
        "pi" => match m.split_once('/') { Some((p, mm)) => vec!["--provider".into(), p.into(), "--model".into(), mm.into()], None => vec!["--model".into(), m.into()] },
        k => crate::agents::kinds::kind(k).map(|a| fill(a.model, m)).unwrap_or_default(),
    }
}
pub fn effort_args(kind: &str, e: &str) -> Vec<String> { crate::agents::kinds::kind(kind).map(|a| fill(a.effort, e)).unwrap_or_default() }
/// Codex's Fast mode (every request carries `service_tier: "priority"`); off names the default tier, last on the line.
pub fn fast_args(kind: &str, on: bool) -> Vec<String> {
    match (kind, on) { ("codex", true) => vec!["-c".into(), "service_tier=\"fast\"".into()], ("codex", false) => vec!["-c".into(), "service_tier=\"default\"".into()], _ => vec![] }
}
