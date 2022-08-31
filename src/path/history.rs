//! Read and write to the `$PATH` history.

use clap::crate_name;
use home::home_dir;
use rev_lines::RevLines;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::OpenOptions,
    io::{self, BufReader, Write},
    path::PathBuf,
};

/// Check multiple locations for a `$PATH` history file and return the highest
/// priority one.
pub fn get_history_filepath() -> PathBuf {
    // check if $XDG_CONFIG_HOME is set
    let mut cfg_path = match env::var("XDG_CONFIG_HOME") {
        Ok(dir) => PathBuf::from(dir),
        // if not set, make it the default $HOME/.config
        Err(_) => {
            if let Some(mut dir) = home_dir() {
                dir.push(".config");
                dir
            } else {
                PathBuf::new()
            }
        }
    };

    // get config from within $XDG_CONFIG_HOME
    cfg_path.push(crate_name!());
    cfg_path.push(".path_history");
    cfg_path
}

/// Parse the `$PATH` history.
///
/// For memory constraints, parse the last n lines.
/// If no limit is specified, load the file carefully.
pub fn get_nth_last_revision(n: u128) -> io::Result<OsString> {
    // get the history file
    let history_filepath = get_history_filepath();

    // error out if the path history does not exist
    if !history_filepath.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "History file not found. Nothing to revert to.",
        ));
    }
    let history_file = OpenOptions::new().read(true).open(history_filepath)?;

    // iterate through the lines in reverse order
    let rev_lines = RevLines::new(BufReader::new(history_file)).unwrap();

    // error out if the revision is too far back (not enough history in the path history file)
    let revision_path = match rev_lines.into_iter().nth((n - 1) as usize) {
        Some(s) => OsString::from(s),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                "History does not contain revision {}. Please specify a smaller revision number.",
                &n
            ),
            ))
        }
    };

    Ok(revision_path)
}

/// Append the new `$PATH` to the history file.
pub fn write_to_history(p: &OsStr) -> io::Result<()> {
    // convert into a writable string
    let p_str = p.to_str().unwrap();
    // get the history file
    let history_filepath = get_history_filepath();
    // open the file with the appropriate permissions
    let mut history_file = OpenOptions::new()
        // create it if it doesn't exist
        .create(true)
        // append to the end of the file
        .append(true)
        // which file path to write to
        .open(history_filepath)?;
    writeln!(history_file, "{}", p_str)
}
