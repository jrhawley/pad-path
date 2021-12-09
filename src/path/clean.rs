//! Clean up the `$PATH`.

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

use clap::{crate_authors, AppSettings};
use itertools::Itertools;
use std::{
    env::current_dir,
    fs::canonicalize,
    io,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};
use structopt::StructOpt;

use super::{read::{read_path, combine_path_like}, write::replace_path};

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Remove duplicates and non-existent directories",
    author = crate_authors!(),
    visible_alias = "dedup",
    settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
)]
pub struct CleanOpt {
    /// Don't print warnings when modifying `$PATH`.
    #[structopt(short, long)]
    quiet: bool,

    /// Add current `$PATH` to the history
    #[structopt(short = "H", long)]
    history: bool,

    /// Don't do anything, just preview what this command would do
    #[structopt(short = "n", long = "dry-run")]
    dry_run: bool,
}

impl CleanOpt {
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        Ok(())
    }
}

/// Clean up `$PATH` by removing duplicated directories.
///
/// No behaviour changes occur after cleaning the path, since we keep the first
/// occurrence in its position and remove all latter occurrences.
pub fn clean_path(opts: &CleanOpt) -> io::Result<()> {
    let current_path = read_path();
    // only keep existing and unique directories
    let vpath: Vec<PathBuf> = current_path
        .into_iter()
        .filter(|p| p.exists())
        .unique()
        .collect();
    let newpath = combine_path_like(vpath)?;
    match replace_path(newpath, opts.dry_run, opts.history, opts.quiet) {
        Ok(()) => Ok(()),
        Err(e) => {
            if !opts.quiet {
                eprintln!("Could not clean `$PATH`. {}", e);
            }
            Err(e)
        }
    }
}

/// Clean directory names by removing trailing folder separator characters and
/// converting to absolute paths
pub fn clean_dir_name<P: AsRef<Path>>(dir: P) -> PathBuf {
    let _cleaned_dir = match has_trailing_slash(&dir) {
        true => {
            let mut _temp_dir = dir
                .as_ref()
                .to_string_lossy()
                .trim_end_matches(MAIN_SEPARATOR)
                .to_string();
            PathBuf::from(_temp_dir)
        }
        false => dir.as_ref().to_path_buf(),
    };
    make_abs_path(&_cleaned_dir)
}

/// Clean a list of directories
pub fn clean_dirs_names<P: AsRef<Path>>(dirs: &[P]) -> Vec<PathBuf> {
    dirs.iter().map(clean_dir_name).collect()
}

/// Force a PathBuf to be absolute, or make it absolute using the current directory
fn make_abs_path<P: AsRef<Path>>(p: P) -> PathBuf {
    match p.as_ref().is_relative() {
        true => match p.as_ref().exists() {
            true => canonicalize(p).unwrap(),
            false => {
                let mut abs_dir = current_dir().unwrap();
                abs_dir.push(p);
                abs_dir
            }
        },
        false => PathBuf::from(p.as_ref()),
    }
}

/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "windows")]
fn has_trailing_slash<P: AsRef<Path>>(p: P) -> bool {
    let last = p.as_ref().as_os_str().encode_wide().last();
    // Windows can have '/' or '\' as its trailing character
    last == Some(b'\\' as u16) || last == Some(b'/' as u16)
}
/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "linux")]
fn has_trailing_slash<P: AsRef<Path>>(p: P) -> bool {
    p.as_ref().to_string_lossy().as_bytes().last() == Some(&b'/')
}
/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "macos")]
fn has_trailing_slash<P: AsRef<Path>>(p: P) -> bool {
    p.as_ref().to_string_lossy().as_bytes().last() == Some(&b'/')
}
