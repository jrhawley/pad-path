//! Read the current `$PATH`.

use std::{
    env::{split_paths, var_os},
    ffi::OsString,
    path::PathBuf,
};

use crate::path::clean_dir_name;

/// Get the value for the `$PATH` environment variable.
pub fn read_raw_path() -> Option<OsString> {
    var_os("PATH")
}

/// Get the value for the `$PATH` environment variable, split across a vector.
pub fn read_path() -> Vec<PathBuf> {
    match read_raw_path() {
        Some(path_str) => split_paths(&path_str)
            .into_iter()
            .map(|d| clean_dir_name(d))
            .collect(),
        None => vec![PathBuf::from("")],
    }
}
