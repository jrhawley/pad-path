//! Functions for reading and writing to the PATH history

use std::{env, ffi::OsString, path::PathBuf};

use clap::crate_name;
use home::home_dir;

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
    match cfg_path.exists() {
        true => cfg_path,
        // look for a file in the current directory
        false => PathBuf::from(".path_history"),
    }
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
