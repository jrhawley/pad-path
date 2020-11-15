use itertools::Itertools;
use std::cmp::min;
use std::env::{current_dir, join_paths, split_paths, var_os};
use std::ffi::OsString;
use std::fs::canonicalize;
use std::io::Error;
use std::path::PathBuf;

/// Get the value for the PATH environment variable
fn read_raw_path() -> String {
    match var_os("PATH") {
        Some(p_str) => String::from(p_str.to_str().unwrap()),
        None => String::new(),
    }
}

/// Get the value for the PATH environment variable, split across a vector
pub fn read_path() -> Vec<PathBuf> {
    let path_str = read_raw_path();
    split_paths(&path_str).into_iter().collect()
}

/// Replace the PATH environment variabel
fn replace_path(newpath: OsString, dryrun: bool) -> Result<(), Error> {
    // need to use Registry Editor to edit environment variables on windows
    //     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    //     let (env, _) = hkcu.create_subkey("Environment").unwrap();
    let current_path = read_raw_path();
    if dryrun {
        println!("PATH before modifcation:\n\t{}", current_path);
        println!("PATH after modifcation:\n\t{}", newpath.to_str().unwrap());
    } else {
        // env::set_var("PATH", newpath);
    }
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
pub fn add_to_path(dir: PathBuf, prepend: bool, dryrun: bool) -> Result<(), Error> {
    // read the path and convert into Vec<&PathBuf>
    let mut current_path: Vec<PathBuf> = read_path();
    let newpath = match prepend {
        true => {
            let mut all_paths = vec![dir];
            all_paths.append(&mut current_path);
            join_paths(all_paths).unwrap()
        }
        false => {
            current_path.push(dir);
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
        panic!("Directory is not found in PATH. It will not be removed.")
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
        panic!("Directory is not found in PATH. Nothing is changed.")
    }
}

/// Clean up PATH by removing duplicated directories.
/// No behaviour changes, since we keep the first occurrence in its position and remove all
/// latter occurrences.
pub fn clean_path(dryrun: bool) -> Result<(), Error> {
    let current_path = read_path();
    let vpath: Vec<PathBuf> = current_path.into_iter().unique().collect();
    let newpath = join_paths(vpath).unwrap();
    replace_path(newpath, dryrun)
}
