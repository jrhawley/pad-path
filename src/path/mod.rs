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
pub mod clean;
pub mod history;
pub mod read;
pub mod remove;
pub mod write;

use self::history::get_nth_last_revision;
use self::{read::read_path, write::replace_path};

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

/// Revert to an earlier PATH
/// This makes use of the `.path_history` file
pub fn revert_path(revision: u128, dry_run: bool, add_to_history: bool) -> Result<(), Error> {
    // look up an old `$PATH` from the path history
    let newpath = get_nth_last_revision(revision)?;

    // replace the current path with the revised one
    replace_path(newpath, dry_run, add_to_history)
}
