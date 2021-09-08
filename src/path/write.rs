//! Functions for writing out the modified PATH.

use std::ffi::OsString;
use std::io::Error;

use crate::path::read::read_raw_path;

/// Replace the PATH environment variable
#[cfg(target_os = "windows")]
pub fn replace_path(newpath: OsString, dry_run: bool) -> Result<(), Error> {
    let current_path = String::from(read_raw_path().unwrap().to_str().unwrap());
    let _new_path = String::from(newpath.to_str().unwrap());
    if dry_run {
        eprintln!("PATH before modification:\n\t{}", &current_path);
        eprintln!("PATH after modification:\n\t{}", &_new_path);
        // skip the remainder of the function
        return Ok(());
    }
    println!("{}", &_new_path);
    Ok(())
}

/// Replace the PATH environment variable
#[cfg(not(target_os = "windows"))]
pub fn replace_path(newpath: OsString, dry_run: bool) -> Result<(), Error> {
    let current_path = String::from(read_raw_path().unwrap().to_str().unwrap());
    let _new_path = String::from(newpath.to_str().unwrap());
    if dry_run {
        eprintln!("PATH before modification:\n\t{}", &current_path);
        eprintln!("PATH after modification:\n\t{}", &_new_path);
        // skip the remainder of the function
        return Ok(());
    }
    println!("{}", &_new_path);
    Ok(())
}
