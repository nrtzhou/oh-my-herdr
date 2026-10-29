//! The Watcher's one-shot: a headless command that answers once and ends, one per program.
use crate::agents::kinds::fill;
use crate::agents::adapter::launch::model_args;
/// A headless command for the Watcher: the program, its arguments, and what goes in on its stdin.
pub struct OneShot { pub command: String, pub args: Vec<String>, pub stdin: Option<String> }
pub fn one_shot(kind: &str, model: &str, effort: &str, prompt: &str, system: &str, lean: bool) -> Option<OneShot> {
    match kind {
        // The prompt goes in on stdin: --tools takes a list and would swallow a trailing prompt. `lean` starts it with
        // nothing of yours loaded and the helper's few words in place of Claude Code's own long instructions.
        "claude" => {
            let mut a: Vec<String> = vec!["-p".into()];
            if !model.is_empty() { a.extend(["--model".into(), model.into()]); }
            if !effort.is_empty() { a.extend(["--effort".into(), effort.into()]); }
            if lean { a.extend(["--safe-mode", "--strict-mcp-config", "--disable-slash-commands"].map(String::from)); }
            if !system.is_empty() { a.extend([if lean { "--system-prompt" } else { "--append-system-prompt" }.to_string(), system.into()]); }
            a.extend(["--no-session-persistence", "--tools", ""].map(String::from));
            Some(OneShot { command: "claude".into(), args: a, stdin: Some(prompt.into()) })
        }
        "codex" => {
            let mut a: Vec<String> = ["exec", "--skip-git-repo-check", "--ephemeral", "--color", "never"].map(String::from).to_vec();
            if !model.is_empty() { a.extend(["-m".into(), model.into()]); }
            a.push(prompt.into());
            Some(OneShot { command: "codex".into(), args: a, stdin: None })
        }
        "pi" => {
            let mut a: Vec<String> = ["-p", "--no-session", "--no-tools", "--no-skills", "--no-context-files"].map(String::from).to_vec();
            if !model.is_empty() { a.extend(model_args("pi", model)); }
            if !system.is_empty() { a.extend(["--append-system-prompt".into(), system.into()]); }
            a.extend(["--".into(), prompt.into()]);
            Some(OneShot { command: "pi".into(), args: a, stdin: None })
        }
        // herdr's others: their one-shot, from the table (kinds.rs `headless`); the instructions go above the prompt
        // where the program has no flag for them.
        k => {
            let a = crate::agents::kinds::kind(k)?;
            let h = a.headless.as_ref()?;
            let mut args = fill(h.args, "");
            if !model.is_empty() { args.extend(fill(h.model, model)); }
            let prompt = if !system.is_empty() && h.system.is_empty() { format!("{system}\n\n{prompt}") } else { prompt.to_string() };
            if !system.is_empty() { args.extend(fill(h.system, system)); }
            if h.prompt.is_empty() { return Some(OneShot { command: crate::agents::kinds::program(k).into(), args, stdin: Some(prompt) }); }
            args.extend(fill(h.prompt, &prompt));
            Some(OneShot { command: crate::agents::kinds::program(k).into(), args, stdin: None })
        }
    }
}
/// Can it be the Watcher: does it have a headless one-shot?
pub fn can_one_shot(kind: &str) -> bool { one_shot(kind, "", "", "", "", false).is_some() }
