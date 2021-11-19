//! Write out the modified `$PATH`.

use std::ffi::OsString;
use std::io;

use crate::path::{history::write_to_history, read::read_raw_path};

/// Replace the `$PATH` environment variable.
pub fn replace_path(
    newpath: OsString,
    dry_run: bool,
    add_to_history: bool,
    quiet: bool,
) -> io::Result<()> {
    let current_raw_path = read_raw_path().unwrap();
    let current_path = String::from(current_raw_path.to_str().unwrap());
    let _new_path = String::from(newpath.to_str().unwrap());
    if dry_run && !quiet {
        eprintln!("`$PATH` before modification:\n\t{}", &current_path);
        eprintln!("`$PATH` after modification:\n\t{}", &_new_path);
        // skip the remainder of the function
        return Ok(());
    }
    // if specified, write the old `$PATH` into the history
    if add_to_history {
        write_to_history(&current_raw_path)?;
    }
    println!("{}", &_new_path);

    Ok(())
}
