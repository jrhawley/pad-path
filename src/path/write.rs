//! Write out the modified `$PATH`.

use crate::path::{
    clean::clean_given_path,
    history::write_to_history,
    read::{read_raw_path, split_path_like},
};
use std::ffi::OsString;
use std::io;

/// Replace the `$PATH` environment variable.
pub fn replace_path(
    newpath: OsString,
    dry_run: bool,
    add_to_history: bool,
    quiet: bool,
) -> io::Result<()> {
    let current_raw_path = read_raw_path().unwrap();
    let current_path = String::from(current_raw_path.to_str().unwrap());

    // clean the newpath before printing it
    let cleaned_newpath = String::from(
        clean_given_path(split_path_like(&newpath))?
            .to_str()
            .unwrap(),
    );

    if dry_run && !quiet {
        eprintln!("`$PATH` before modification:\n\t{}", &current_path);
        eprintln!("`$PATH` after modification:\n\t{}", &cleaned_newpath);
        // skip the remainder of the function
        return Ok(());
    }
    // if specified, write the old `$PATH` into the history
    if add_to_history {
        write_to_history(&current_raw_path)?;
    }
    println!("{}", &cleaned_newpath);

    Ok(())
}
