//! Where each program saves its conversations and which one is the newest: the folder of the working directory, the
//! file of a session id, and a conversation found by its prompts when the program names its own.
use crate::agents::homes::*;
use crate::agents::read_turn::{find_file, head_of, when};
use crate::json::*;
use std::fs;
use std::path::{Path, PathBuf};
fn real(p: &Path) -> PathBuf { fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()) }
fn dashed(p: &Path) -> String { p.display().to_string().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect() }
/// Where a conversation is saved.
pub fn transcript(kind: &str, session: &str, cwd: &Path) -> Option<PathBuf> {
    // herdr integrations may report the saved conversation's path instead of its id.
    if Path::new(session).is_file() { return Some(PathBuf::from(session)); }
    match kind {
        "claude" => {
            let direct = claude_home().join("projects").join(dashed(&real(cwd))).join(format!("{session}.jsonl"));
            if direct.exists() { return Some(direct); }
            let want = format!("{session}.jsonl");
            find_file(&claude_home().join("projects"), &|n| n == want, 1).into_iter().next()
        }
        "codex" => { let want = format!("{session}.jsonl"); find_file(&codex_home().join("sessions"), &|n| n.ends_with(&want), 4).into_iter().next() }
        // Also where it was saved before its folder moved (the Coordinator's, under the board's old home).
        "pi" => { let (want, was) = (format!("_{session}.jsonl"), crate::store::move_home::moved_from(cwd)); pi_files(&[Some(cwd), was.as_deref()].into_iter().flatten().collect::<Vec<_>>(), &|n| n.ends_with(&want)).into_iter().next() }
        // omp names the file `<timestamp>_<session id>.jsonl` in the folder of its working directory.
        "omp" => { let (want, was) = (format!("_{session}.jsonl"), crate::store::move_home::moved_from(cwd)); omp_files(&[Some(cwd), was.as_deref()].into_iter().flatten().collect::<Vec<_>>(), &|n| n.ends_with(&want)).into_iter().next() }
        _ => None,
    }
}

/// A saved file path is readable, but never an agent's resume id.
pub fn resume_id(session: &str) -> Option<String> {
    let p = Path::new(session);
    if session.is_empty() || p.is_absolute() || p.components().count() > 1 || p.extension().is_some_and(|e| e == "jsonl") { return None; }
    Some(session.to_string())
}
/// pi keeps each working folder's conversations in a folder of their own, named after its path (`--home-me-app--`).
fn pi_folder(cwd: &Path) -> PathBuf { let p = cwd.display().to_string(); pi_home().join("sessions").join(format!("--{}--", p.strip_prefix(['/', '\\']).unwrap_or(&p).replace(['/', '\\', ':'], "-"))) }
/// omp names the same folders as it writes them itself (`-apps-mine` under home, `-tmp-x` under the temp root,
/// `--abs-path--` elsewhere; canonical, symlinks resolved).
fn omp_folder(cwd: &Path) -> PathBuf {
    fn enc(rel: &Path, prefix: &str) -> String {
        let e = rel.display().to_string().replace(['/', '\\', ':'], "-");
        if e.is_empty() { prefix.to_string() } else if prefix.ends_with('-') { format!("{prefix}{e}") } else { format!("{prefix}-{e}") }
    }
    let p = real(cwd);
    if let Ok(rel) = p.strip_prefix(&real(&crate::store::files::user_home())) { return omp_home().join("sessions").join(enc(rel, "-")); }
    if let Ok(rel) = p.strip_prefix(&real(&std::env::temp_dir())) { return omp_home().join("sessions").join(enc(rel, "-tmp")); }
    let s = p.display().to_string();
    omp_home().join("sessions").join(format!("--{}--", s.strip_prefix(['/', '\\']).unwrap_or(&s).replace(['/', '\\', ':'], "-")))
}
/// pi's conversations of these working folders (as given, and their real paths), newest first: only those folders are
/// read, never every folder of a long history (#393).
fn pi_files(cwds: &[&Path], test: &dyn Fn(&str) -> bool) -> Vec<PathBuf> { files_in(&pi_folder, cwds, test) }
fn omp_files(cwds: &[&Path], test: &dyn Fn(&str) -> bool) -> Vec<PathBuf> { files_in(&omp_folder, cwds, test) }
fn files_in(folder: &dyn Fn(&Path) -> PathBuf, cwds: &[&Path], test: &dyn Fn(&str) -> bool) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![];
    for d in cwds.iter().flat_map(|c| [real(c), c.to_path_buf()]).map(|c| folder(&c)) { if !dirs.contains(&d) { dirs.push(d); } }
    let mut hits: Vec<PathBuf> = dirs.iter().flat_map(|d| find_file(d, test, 0)).collect();
    hits.sort_by_key(|f| std::cmp::Reverse(mtime_ms(f)));
    hits
}
fn mtime_ms(f: &Path) -> Option<i64> { fs::metadata(f).and_then(|m| m.modified()).ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_millis() as i64) }
fn first_line(s: &str) -> &str { s.split('\n').next().unwrap_or("") }
/// The newest conversation saved in a folder since a time: the program's own "new chat" command starts another one.
/// `was`: the path this folder had before it was moved; a program started back then still saves under that one.
pub fn latest_session(kind: &str, cwd: &Path, since: i64, was: Option<&Path>) -> Option<String> {
    let real = real(cwd);
    match kind {
        "claude" => {
            // Only a chat of the interactive program itself: not a helper's side conversation and not a headless run.
            let own = |f: &Path| { let head = head_of(f, 65536); let entry = head.find("\"entrypoint\":\"").map(|i| head[i + 14..].split('"').next().unwrap_or("").to_string()); entry.map(|e| e == "cli").unwrap_or(true) && !first_line(&head).contains("\"isSidechain\":true") };
            let is_id = |n: &str| n.len() == 42 && n.ends_with(".jsonl") && n[..36].chars().all(|c| c.is_ascii_hexdigit() || c == '-') && n.as_bytes()[8] == b'-';
            for dir in [Some(real.as_path()), was].into_iter().flatten() {
                for f in find_file(&claude_home().join("projects").join(dashed(dir)), &is_id, 0) {
                    if mtime_ms(&f).map(|m| m < since).unwrap_or(true) { break; }
                    if own(&f) { return f.file_stem().map(|s| s.to_string_lossy().into_owned()); }
                }
            }
            None
        }
        "codex" => {
            for f in find_file(&codex_home().join("sessions"), &|n| n.ends_with(".jsonl"), 4) {
                if mtime_ms(&f).map(|m| m < since).unwrap_or(true) { return None; }
                let head = head_of(&f, 400000);
                let Ok(meta) = serde_json::from_str::<Value>(first_line(&head)) else { continue };
                let p = meta.at("payload");
                let cwd_ok = p.s("cwd") == real.display().to_string() || was.map(|w| p.s("cwd") == w.display().to_string()).unwrap_or(false);
                let source_ok = p.at("source").is_null() || (p.at("source").is_string() && p.s("source") != "exec");
                if cwd_ok && p.b("id") && !p.b("parent_thread_id") && source_ok { return Some(p.t("id")); }
            }
            None
        }
        "pi" => {
            for f in pi_files(&[Some(cwd), was].into_iter().flatten().collect::<Vec<_>>(), &|n| n.ends_with(".jsonl")) {
                if mtime_ms(&f).map(|m| m < since).unwrap_or(true) { return None; }
                let head = head_of(&f, 8192);
                let Ok(meta) = serde_json::from_str::<Value>(first_line(&head)) else { continue };
                let cwd_ok = meta.s("cwd") == real.display().to_string() || meta.s("cwd") == cwd.display().to_string() || was.map(|w| meta.s("cwd") == w.display().to_string()).unwrap_or(false);
                if meta.s("type") == "session" && meta.b("id") && !meta.b("parentSession") && cwd_ok { return Some(meta.t("id")); }
            }
            None
        }
        // omp's first line is a title slot of its own; the session's own entry is the second.
        "omp" => {
            for f in omp_files(&[Some(cwd), was].into_iter().flatten().collect::<Vec<_>>(), &|n| n.ends_with(".jsonl")) {
                if mtime_ms(&f).map(|m| m < since).unwrap_or(true) { return None; }
                let head = head_of(&f, 8192);
                let Some(Ok(meta)) = head.lines().nth(1).map(|l| serde_json::from_str::<Value>(l)) else { continue };
                let cwd_ok = meta.s("cwd") == real.display().to_string() || meta.s("cwd") == cwd.display().to_string() || was.map(|w| meta.s("cwd") == w.display().to_string()).unwrap_or(false);
                if meta.s("type") == "session" && meta.b("id") && !meta.b("parentSession") && cwd_ok { return Some(meta.t("id")); }
            }
            None
        }
        _ => None,
    }
}
/// Codex and omp name their own conversations: without a hook, the one for this task is the file made after launch,
/// in this folder, whose prompts name this task and role. More than one: the one begun since launch (a review started
/// again has the same words as the one before it). No match, or still more than one: none.
pub fn find_session(kind: &str, cwd: &Path, since: i64, needle: &str) -> Option<String> { find_session_but(kind, cwd, since, needle, "") }
/// The same, leaving out a conversation already known to be another's (`but`: the one a review started again had).
pub fn find_session_but(kind: &str, cwd: &Path, since: i64, needle: &str, but: &str) -> Option<String> {
    let hits: Vec<(i64, String)> = match kind {
        "codex" => find_file(&codex_home().join("sessions"), &|n| n.ends_with(".jsonl"), 4).into_iter()
            .filter(|f| mtime_ms(f).map(|m| m >= since - 60000).unwrap_or(false))
            .filter(|f| !crate::agents::helpers::codex_helper_file(f)) // a helper's conversation carries the agent's first prompt along
            .filter_map(|f| { let head = head_of(&f, 400000); let m = serde_json::from_str::<Value>(first_line(&head)).ok()?; let at = when(m.at("timestamp")); (m.at("payload").s("cwd") == real(cwd).display().to_string() && at >= since - 5000 && head.contains(needle) && (but.is_empty() || m.at("payload").t("id") != but)).then(|| (at, m.at("payload").t("id"))) })
            .collect(),
        // omp's second line (the session's own entry) says whose it is; a sub-agent's names its parent and is left
        // out, as a Codex helper's is.
        "omp" => omp_files(&[cwd], &|n| n.ends_with(".jsonl")).into_iter()
            .filter(|f| mtime_ms(f).map(|m| m >= since - 60000).unwrap_or(false))
            .filter_map(|f| { let head = head_of(&f, 400000); let m: Value = serde_json::from_str(head.lines().nth(1)?).ok()?; let at = when(m.at("timestamp")); (m.s("type") == "session" && m.b("id") && !m.b("parentSession") && m.s("cwd") == real(cwd).display().to_string() && at >= since - 5000 && head.contains(needle) && (but.is_empty() || m.t("id") != but)).then(|| (at, m.t("id"))) })
            .collect(),
        _ => return None,
    };
    let since_launch: Vec<&(i64, String)> = hits.iter().filter(|h| h.0 >= since).collect();
    match (hits.len(), since_launch.len()) { (1, _) => Some(hits[0].1.clone()), (_, 1) => Some(since_launch[0].1.clone()), _ => None }
}
pub fn can_find_session(kind: &str) -> bool { matches!(kind, "codex" | "omp") }
