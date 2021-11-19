//! Read, write, and process the current `$PATH`.

use itertools::Itertools;
use std::cmp::min;
use std::env::{current_dir, join_paths};
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

pub mod add;
pub mod history;
pub mod read;
pub mod write;

use self::history::get_nth_last_revision;
use self::{read::read_path, write::replace_path};

/// Remove the given directory to the `$PATH` environment variable
pub fn rm_from_path(dir: PathBuf, dry_run: bool, add_to_history: bool) -> Result<(), Error> {
    let current_path = read_path();
    let idx = current_path.iter().position(|x| *x == dir);
    // if the directory is found within PATH
    if let Some(i) = idx {
        let mut vpath = current_path.clone();
        vpath.remove(i);
        let newpath = join_paths(vpath).unwrap();
        replace_path(newpath, dry_run, add_to_history)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Directory `{}` not found in `$PATH`. No changes made.",
                dir.display()
            ),
        ))
    }
}

/// Change the priority of a directory by moving it earlier or later in PATH
pub fn change_priority(
    dir: PathBuf,
    jump: i8,
    dry_run: bool,
    add_to_history: bool,
) -> Result<(), Error> {
    let current_path = read_path();
    let idx = current_path.iter().position(|x| *x == dir);
    // if the directory is found within PATH
    if let Some(i) = idx {
        // calculate the new position for `dir`, and ensure that it is within the appropriate bounds
        let i_signed = i as i8;
        let new_idx: usize;
        if i_signed + jump < 0 {
            new_idx = 0;
        } else {
            new_idx = min((i_signed + jump) as usize, current_path.len() - 1);
        }
        let mut vpath: Vec<PathBuf>;

        // if moving to a higher priority
        if jump < 0 {
            // get the first few elements of PATH
            vpath = (0..new_idx)
                .into_iter()
                .map(|j| current_path[j].clone())
                .collect();
            // move `dir` into the next position
            vpath.push(dir);
            // add remaining elements
            vpath.append(
                &mut (new_idx..i)
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect(),
            );
            vpath.append(
                &mut ((i + 1)..current_path.len())
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect(),
            );
        // if no change, do nothing
        } else if jump == 0 {
            vpath = current_path.clone();
        // if moving to a lower priority
        } else {
            // get the first few elements of PATH
            vpath = (0..i)
                .into_iter()
                .map(|j| current_path[j].clone())
                .collect();
            vpath.append(
                &mut ((i + 1)..(new_idx + 1))
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect(),
            );
            // move `dir` into the next position
            vpath.push(dir);
            // add remaining elements
            vpath.append(
                &mut ((new_idx + 1)..current_path.len())
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect(),
            );
        }
        let newpath = join_paths(vpath).unwrap();
        replace_path(newpath, dry_run, add_to_history)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Directory `{}` not found in `$PATH`. No changes made.",
                dir.display()
            ),
        ))
    }
}

/// Clean up `$PATH` by removing duplicated directories.
/// No behaviour changes occur after cleaning the path, since we keep the first
/// occurrence in its position and remove all latter occurrences.
pub fn clean_path(dry_run: bool, add_to_history: bool) -> Result<(), Error> {
    let current_path = read_path();
    // only keep existing and unique directories
    let vpath: Vec<PathBuf> = current_path
        .into_iter()
        .filter(|p| p.exists())
        .unique()
        .collect();
    let newpath = join_paths(vpath).unwrap();
    replace_path(newpath, dry_run, add_to_history)
}

/// Clean directory names by removing trailing folder separator characters and
/// converting to absolute paths
fn clean_dir_name<P: AsRef<Path>>(dir: P) -> PathBuf {
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

/// Revert to an earlier PATH
/// This makes use of the `.path_history` file
pub fn revert_path(revision: u128, dry_run: bool, add_to_history: bool) -> Result<(), Error> {
    // look up an old `$PATH` from the path history
    let newpath = get_nth_last_revision(revision)?;

    // replace the current path with the revised one
    replace_path(newpath, dry_run, add_to_history)
}
