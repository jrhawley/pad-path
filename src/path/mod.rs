//! Read, write, and process the current `$PATH`.

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

pub mod add;
pub mod clean;
pub mod history;
pub mod priority;
pub mod read;
pub mod remove;
pub mod revert;
pub mod write;
