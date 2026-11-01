//! How each program is started and found again: its turn-end hook, the conversation id it is given, the standing
//! rules, where it saves a conversation and which one is its newest, a headless call for the Watcher, and marking a
//! repo as trusted in its own settings. One match per question, one arm per program (kinds.rs has the plain facts);
//! one area per file in adapter/, and this one passes their names on.
pub mod hook;
pub mod launch;
pub mod one_shot;
pub mod sessions;
pub mod trust;

pub use hook::{hook_args, has_hook, Hook};
pub(crate) use hook::write_private;
pub use launch::{effort_args, fast_args, fresh_args, has_standing, model_args, new_session, resume_args, session_args, standing};
pub use one_shot::{can_one_shot, one_shot, OneShot};
pub use sessions::{can_find_session, find_session, find_session_but, latest_session, resume_id, transcript};
pub use trust::{can_trust, skill_dir, trust};
