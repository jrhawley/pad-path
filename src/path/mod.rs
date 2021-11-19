//! Read, write, and process the current `$PATH`.

use std::io;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

pub mod add;
pub mod clean;
pub mod history;
pub mod priority;
pub mod read;
pub mod remove;
pub mod write;

use self::{history::get_nth_last_revision, write::replace_path};

/// Revert to an earlier PATH
/// This makes use of the `.path_history` file
pub fn revert_path(revision: u128, dry_run: bool, add_to_history: bool) -> io::Result<()> {
    // look up an old `$PATH` from the path history
    let newpath = get_nth_last_revision(revision)?;

    // replace the current path with the revised one
    replace_path(newpath, dry_run, add_to_history)
}
