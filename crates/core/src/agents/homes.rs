//! Where each agent program keeps its own settings and files.
use crate::store::files::user_home;
use std::path::PathBuf;

fn env(k: &str) -> Option<PathBuf> { std::env::var(k).ok().filter(|v| !v.is_empty()).map(PathBuf::from) }
pub fn claude_file() -> PathBuf { env("OMH_CLAUDE_JSON").unwrap_or_else(|| user_home().join(".claude.json")) }
pub fn codex_file() -> PathBuf { env("OMH_CODEX_TOML").unwrap_or_else(|| codex_home().join("config.toml")) }
pub fn claude_home() -> PathBuf { env("CLAUDE_CONFIG_DIR").unwrap_or_else(|| user_home().join(".claude")) }
pub fn codex_home() -> PathBuf { env("CODEX_HOME").unwrap_or_else(|| user_home().join(".codex")) }
pub fn pi_home() -> PathBuf { env("PI_CODING_AGENT_DIR").unwrap_or_else(|| user_home().join(".pi").join("agent")) }
/// omp (oh-my-pi) keeps everything under one agent directory; it is a pi build, so pi's own variable names work too.
pub fn omp_home() -> PathBuf { env("OMP_CODING_AGENT_DIR").or_else(|| env("PI_CODING_AGENT_DIR")).unwrap_or_else(|| user_home().join(".omp").join("agent")) }
