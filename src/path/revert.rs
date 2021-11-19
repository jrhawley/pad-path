//! Revert `$PATH` to a previous value.

use std::io;

use super::{history::get_nth_last_revision, write::replace_path};

/// Revert to an earlier `$PATH`
///
/// This makes use of the `.path_history` file
pub fn revert_path(revision: u128, dry_run: bool, add_to_history: bool) -> io::Result<()> {
    // look up an old `$PATH` from the path history
    let newpath = get_nth_last_revision(revision)?;

    // replace the current path with the revised one
    replace_path(newpath, dry_run, add_to_history)
}
