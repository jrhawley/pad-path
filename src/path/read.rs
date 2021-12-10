//! Read the current `$PATH`.

use std::{
    env::{split_paths, var_os, join_paths},
    ffi::{OsString, OsStr},
    io,
    path::PathBuf,
};

use crate::path::clean::clean_dir_name;

/// Get the value for the `$PATH` environment variable.
pub fn read_raw_path() -> Option<OsString> {
    var_os("PATH")
}

/// Get the value for the `$PATH` environment variable, split across a vector.
pub fn read_path() -> Vec<PathBuf> {
    match read_raw_path() {
        Some(path_str) => split_path_like(&path_str),
        None => vec![PathBuf::from("")],
    }
}

/// Split an `OsString` formatted like a `$PATH` into a `Vec`.
/// 
/// This is a helper function for a few different others.
pub fn split_path_like(s: &OsStr) -> Vec<PathBuf> {
    split_paths(s)
        .into_iter()
        .map(clean_dir_name)
        .collect()
}

/// Combine a multiple directories back into a single `$PATH`-like `OsString`.
// pub fn combine_path_like<P: AsRef<Path>>(dirs: &[P]) -> io::Result<OsString> {
pub fn combine_path_like(dirs: Vec<PathBuf>) -> io::Result<OsString> {
    match join_paths(dirs) {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            e,
        ))
    }
}