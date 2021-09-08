//! Functions for reading and writing to the PATH history

use clap::crate_name;
use home::home_dir;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

/// Check multiple locations for a PATH history file and return the highest priority one
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

/// Parse the PATH history
/// For memory constraints, parse the last n lines.
/// If not limit is specified, load the file carefully.
fn parse_history(n: Option<u128>) -> Vec<OsString> {
    // get the history file
    let history_filepath = get_history_filepath();
    //
    todo!()
}

/// Write the new PATH to the history file
pub fn write_to_history(p: &OsStr) -> Result<(), io::Error> {
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
