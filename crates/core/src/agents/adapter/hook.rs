//! The turn-end hook: what a program is launched with so it calls the board's `turn-ended` at every turn end (a
//! settings file for Claude Code, Codex's notify, a one-file extension for pi and omp). The hook's file is written
//! for the task alone, private.
use crate::agents::homes::codex_file;
use crate::err::Res;
use crate::json::*;
use crate::util::shq;
use std::fs;
use std::path::{Path, PathBuf};
/// The hook's call back into the board, at each turn end.
pub struct Hook { pub shim: String, pub id: String, pub role: String, pub token: String, pub round: i64, pub dir: PathBuf }
fn hook_command(h: &Hook) -> Vec<String> {
    let mut c = vec![h.shim.clone(), "turn-ended".into(), "--id".into(), h.id.clone(), "--role".into(), h.role.clone(), "--token".into(), h.token.clone()];
    if h.role == "reviewer" { c.extend(["--round".to_string(), h.round.to_string()]); }
    c
}
pub(crate) fn write_private(file: &Path, text: &str) -> Res<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(file)?;
    std::io::Write::write_all(&mut f, text.as_bytes())?;
    Ok(())
}
/// Launch arguments that make the program call `turn-ended` at every turn end. None: no hook for it here.
pub fn hook_args(kind: &str, h: &Hook) -> Res<Option<Vec<String>>> {
    match kind {
        // A settings file of the task's own, added to the user's settings for this one launch (the user's own hooks
        // still run). The hooks are asynchronous, so a slow or failing one never holds the agent's turn. Idle
        // compaction is off: Claude Code would otherwise shrink a finished task's session left waiting an hour, which
        // herdr sees as a turn.
        "claude" => {
            let command = hook_command(h).iter().map(|s| shq(s)).collect::<Vec<_>>().join(" ");
            let file = h.dir.join(format!("hook-{}.json", h.role));
            let one = json!([{ "hooks": [{ "type": "command", "command": command, "async": true }] }]);
            write_private(&file, &json!({ "hooks": { "Stop": one, "StopFailure": one }, "idleCompaction": false }).to_string())?;
            Ok(Some(vec!["--settings".into(), file.display().to_string()]))
        }
        // Codex runs one `notify` program at each turn end. Given at launch it would replace one the user configured,
        // so a user who has one gets no hook (the hook-less route covers them).
        "codex" => {
            if let Ok(t) = fs::read_to_string(codex_file()) {
                let head = t.split("\n[").next().unwrap_or("");
                if head.lines().any(|l| l.trim_start().starts_with("notify") && l.trim_start()["notify".len()..].trim_start().starts_with('=')) { return Ok(None); }
            }
            Ok(Some(vec!["-c".into(), format!("notify={}", serde_json::to_string(&hook_command(h))?)]))
        }
        // A one-file extension of the task's own: pi's settled event runs the same command, with the session file on stdin.
        "pi" => {
            let file = h.dir.join(format!("hook-{}.ts", h.role));
            let cmd = serde_json::to_string(&hook_command(h))?;
            write_private(&file, &format!(r#"import {{ spawn }} from "node:child_process";
const cmd = {cmd};
export default function (pi: any) {{
  pi.on("agent_settled", (_e: any, ctx: any) => {{
    try {{
      let file = ""; try {{ file = ctx?.sessionManager?.getSessionFile?.() || ""; }} catch {{}}
      const c = spawn(cmd[0], cmd.slice(1), {{ stdio: ["pipe", "ignore", "ignore"], detached: true }});
      c.on("error", () => {{}}); c.stdin.on("error", () => {{}});
      c.stdin.end(JSON.stringify({{ transcript_path: file }})); c.unref();
    }} catch {{}}
  }});
}}
"#))?;
            Ok(Some(vec!["-e".into(), file.display().to_string()]))
        }
        // A one-file extension of the task's own: omp's agent_end event runs the same command, with the session file
        // on stdin. It says `willContinue` when the program goes on by itself: that is not a turn's end. Two more
        // events tell the board what holds the turn up: a task sub-agent going to work (its name, on the task), and
        // the session settling, where the board may ask for one turn more (its answer comes back on the stdout).
        // Three more events are one greyed line each for the chat, no turn of their own: a task's helper back with
        // what it said (the `task` tool's result, its first line), an automatic compaction, an automatic retry.
        "omp" => {
            let file = h.dir.join(format!("hook-{}.ts", h.role));
            let cmd = serde_json::to_string(&hook_command(h))?;
            write_private(&file, &format!(r#"import {{ spawn }} from "node:child_process";
const cmd = {cmd};
function send(payload, done) {{
  try {{
    const c = spawn(cmd[0], cmd.slice(1), {{ stdio: ["pipe", done ? "pipe" : "ignore", "ignore"], detached: true }});
    c.on("error", () => {{}}); if (c.stdin) c.stdin.on("error", () => {{}});
    if (done && c.stdout) {{ let out = ""; c.stdout.on("data", (d) => {{ out += d; }}); c.on("close", () => {{ try {{ done(out); }} catch {{}} }}); }}
    c.stdin.end(JSON.stringify(payload)); c.unref();
  }} catch {{}}
}}
function file(ctx) {{ try {{ return (ctx && ctx.sessionManager && ctx.sessionManager.getSessionFile && ctx.sessionManager.getSessionFile()) || ""; }} catch {{ return ""; }} }}
export default function (omp) {{
  omp.on("agent_end", (_e, ctx) => {{
    try {{ if (_e && _e.willContinue) return; send({{ transcript_path: file(ctx) }}); }} catch {{}}
  }});
  omp.on("before_subagent_spawn", (_e, ctx) => {{
    try {{ const m = _e && _e.patterns; const one = Array.isArray(m) ? (m[0] || "") : (m || ""); send({{ transcript_path: file(ctx), subagent: {{ agent: (_e && _e.agent) || "", model: typeof one === "string" ? one : "" }} }}); }} catch {{}}
  }});
  omp.on("session_stop", (_e, ctx) => {{
    try {{
      return new Promise((resolve) => {{
        let done = false;
        const finish = (v) => {{ if (!done) {{ done = true; try {{ resolve(v && v.continue ? {{ continue: true, additionalContext: v.additionalContext || "" }} : {{}}); }} catch {{ resolve({{}}); }} }} }};
        const t = setTimeout(() => finish({{}}), 2500);
        send({{ transcript_path: file(ctx), session_stop: true }}, (out) => {{
          clearTimeout(t);
          try {{ finish(JSON.parse((out || "").trim().split("\n").filter(Boolean).pop() || "{{}}")); }} catch {{ finish({{}}); }}
        }});
      }});
    }} catch {{ return {{}}; }}
  }});
  omp.on("tool_result", (_e, ctx) => {{
    try {{
      if (!_e || _e.toolName !== "task" || _e.isError) return;
      const i = _e.input || {{}};
      const one = Array.isArray(i.tasks) ? (i.tasks[0] || {{}}) : i;
      const said = (Array.isArray(_e.content) ? _e.content : []).filter((c) => c && c.type === "text").map((c) => c.text || "").join("\n");
      const line = (said.split("\n").map((l) => l.trim()).find(Boolean) || "").slice(0, 120);
      send({{ transcript_path: file(ctx), helper_back: {{ agent: (one && one.name) || "", note: line }} }});
    }} catch {{}}
  }});
  omp.on("auto_compaction_start", (_e, ctx) => {{ try {{ send({{ transcript_path: file(ctx), compaction: "start" }}); }} catch {{}} }});
  omp.on("auto_compaction_end", (_e, ctx) => {{ try {{ send({{ transcript_path: file(ctx), compaction: "end" }}); }} catch {{}} }});
  omp.on("auto_retry_start", (_e, ctx) => {{ try {{ send({{ transcript_path: file(ctx), retry_start: true }}); }} catch {{}} }});
}}
"#))?;
            Ok(Some(vec!["-e".into(), file.display().to_string()]))
        }
        _ => Ok(None),
    }
}
pub fn has_hook(kind: &str) -> bool { matches!(kind, "claude" | "codex" | "pi" | "omp") }
