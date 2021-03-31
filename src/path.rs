use itertools::Itertools;
use std::{cmp::min, collections::HashSet};
use std::env::{current_dir, join_paths, split_paths, var_os};
use std::ffi::OsString;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// Get the value for the PATH environment variable
fn read_raw_path() -> Option<OsString> {
    var_os("PATH")
}

/// Get the value for the PATH environment variable, split across a vector
pub fn read_path() -> Vec<PathBuf> {
    match read_raw_path() {
        Some(path_str) => split_paths(&path_str).into_iter().collect(),
        None => vec![PathBuf::from("")],
    }
}

/// Replace the PATH environment variable on Windows
#[cfg(target_os = "windows")]
fn replace_path(newpath: OsString, dryrun: bool) -> Result<(), Error> {
    let current_path = String::from(read_raw_path().unwrap().to_str().unwrap());
    let _new_path = String::from(newpath.to_str().unwrap());
    if dryrun {
        eprintln!("PATH before modification:\n\t{}", &current_path);
        eprintln!("PATH after modification:\n\t{}", &_new_path);
        // skip the remainder of the function
        return Ok(());
    }
    println!("{}", &_new_path);
    Ok(())
}

/// Replace the PATH environment variable on non-Windows devices
#[cfg(not(target_os = "windows"))]
fn replace_path(newpath: OsString, dryrun: bool) -> Result<(), Error> {
    let current_path = String::from(read_raw_path().unwrap().to_str().unwrap());
    let _new_path = String::from(newpath.to_str().unwrap());
    if dryrun {
        eprintln!("PATH before modification:\n\t{}", &current_path);
        eprintln!("PATH after modification:\n\t{}", &_new_path);
        // skip the remainder of the function
        return Ok(());
    }
    println!("{}", &_new_path);
    Ok(())
}

/// Force a PathBuf to be absolute, or make it absolute using the current directory
pub fn make_abs_path(p: &PathBuf) -> PathBuf {
    match p.is_relative() {
        true => match p.exists() {
            true => canonicalize(p).unwrap(),
            false => {
                let mut abs_dir = current_dir().unwrap();
                abs_dir.push(p);
                abs_dir
            }
        },
        false => (*p).clone(),
    }
}

/// Add the given directory to the PATH environment variable
pub fn add_to_path(dirs: &mut Vec<PathBuf>, prepend: bool, dryrun: bool) -> Result<(), Error> {
    // read the path and convert into Vec<&PathBuf>
    let mut current_path: Vec<PathBuf> = read_path();

    // check that the directories to be added don't alread exist in the PATH
    let _current_dirs: HashSet<PathBuf> = current_path.iter().map(|d| d.clone()).collect();
    let _new_dirs: HashSet<PathBuf> = dirs.iter().map(|d| d.clone()).collect();
    if !_current_dirs.is_disjoint(&_new_dirs) {
        return Err(Error::new(ErrorKind::AlreadyExists, "Directory already exists in PATH. Use `pad up/dn` to change priority of this directory."));
    }
    let newpath = match prepend {
        true => {
            dirs.append(&mut current_path);
            join_paths(dirs).unwrap()
        }
        false => {
            current_path.append(dirs);
            join_paths(current_path).unwrap()
        }
    };
    replace_path(newpath, dryrun)
}

/// Remove the given directory to the PATH environment variable
pub fn rm_from_path(dir: PathBuf, dryrun: bool) -> Result<(), Error> {
    let current_path = read_path();
    let idx = current_path.iter().position(|x| *x == dir);
    // if the directory is found within PATH
    if let Some(i) = idx {
        let mut vpath = current_path.clone();
        vpath.remove(i);
        let newpath = join_paths(vpath).unwrap();
        replace_path(newpath, dryrun)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Directory not found in PATH. No changes made.",
        ))
    }
}

/// Change the priority of a directory by moving it earlier or later in PATH
pub fn change_priority(dir: PathBuf, jump: i8, dryrun: bool) -> Result<(), Error> {
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
        replace_path(newpath, dryrun)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Directory not found in PATH. No changes made.",
        ))
    }
}

/// Clean up PATH by removing duplicated directories.
/// No behaviour changes, since we keep the first occurrence in its position and remove all
/// latter occurrences.
pub fn clean_path(dryrun: bool) -> Result<(), Error> {
    let current_path = read_path();
    // only keep existing and unique directories
    let vpath: Vec<PathBuf> = current_path
        .into_iter()
        .filter(|p| p.exists())
        .unique()
        .collect();
    let newpath = join_paths(vpath).unwrap();
    replace_path(newpath, dryrun)
}
