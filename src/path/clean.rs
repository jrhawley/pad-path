//! Clean up the `$PATH`.

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

use super::{
    read::{combine_path_like, read_path},
    write::replace_path,
};
use clap::{crate_authors, Parser};
use itertools::Itertools;
use std::{
    env::current_dir,
    ffi::OsString,
    fs::canonicalize,
    io,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

#[derive(Debug, Parser)]
#[clap(
    about = "Remove duplicates and non-existent directories",
    author = crate_authors!(),
    visible_alias = "dedup",
)]
pub struct CleanOpt {
    /// Don't print warnings when modifying `$PATH`.
    #[clap(short, long)]
    quiet: bool,

    /// Add current `$PATH` to the history
    #[clap(short = 'H', long)]
    history: bool,

    /// Don't do anything, just preview what this command would do
    #[clap(short = 'n', long = "dry-run")]
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
    let newpath = clean_given_path(current_path)?;
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

/// Clean up a given list of directories by checking for duplicates and other errors.
pub fn clean_given_path(dirs: Vec<PathBuf>) -> io::Result<OsString> {
    // only keep existing and unique directories
    let cleaned_dirs = dirs.into_iter().filter(|p| p.exists()).unique().collect();

    combine_path_like(cleaned_dirs)
}

/// Clean directory names by removing trailing folder separator characters and
/// converting to absolute paths
pub fn clean_dir_name(dir: &Path) -> PathBuf {
    let _cleaned_dir = match has_trailing_slash(&dir) {
        true => {
            let mut _temp_dir = dir
                .to_string_lossy()
                .trim_end_matches(MAIN_SEPARATOR)
                .to_string();
            PathBuf::from(_temp_dir)
        }
        false => dir.to_path_buf(),
    };
    make_abs_path(&_cleaned_dir)
}

/// Clean a list of directories
pub fn clean_dirs_names<P: AsRef<Path>>(dirs: &[P]) -> Vec<PathBuf> {
    dirs.iter().map(|p| clean_dir_name(p.as_ref())).collect()
}

/// Force a PathBuf to be absolute, or make it absolute using the current directory
fn make_abs_path(p: &Path) -> PathBuf {
    match canonicalize(p) {
        Ok(p) => p,
        Err(_) => {
            let mut abs_dir = current_dir().unwrap();
            abs_dir.push(p);
            abs_dir
        }
    }
}

/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "windows")]
fn has_trailing_slash(p: &Path) -> bool {
    let last = p.as_os_str().encode_wide().last();
    // Windows can have '/' or '\' as its trailing character
    last == Some(b'\\' as u16) || last == Some(b'/' as u16)
}
/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "linux")]
fn has_trailing_slash(p: &Path) -> bool {
    p.to_string_lossy().as_bytes().last() == Some(&b'/')
}
/// Check if a directory Path contains the trailing separator.
#[cfg(target_os = "macos")]
fn has_trailing_slash(p: &Path) -> bool {
    p.to_string_lossy().as_bytes().last() == Some(&b'/')
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::path::clean::make_abs_path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn check_make_abs_path(input: &Path, expected: PathBuf) {
        let observed = make_abs_path(input);
        assert_eq!(expected, observed);
    }

    #[test]
    #[cfg(not(windows))]
    fn remove_middle_relative() {
        assert_eq!(
            make_abs_path(Path::new("/usr/../usr")),
            PathBuf::from("/usr")
        );
    }

    #[test]
    #[cfg(windows)]
    fn remove_middle_relative() {
        assert_eq!(
            make_abs_path(Path::new("C:/Users/../Users")),
            PathBuf::from("\\\\?\\C:\\Users")
        );
    }

    #[test]
    #[cfg(not(windows))]
    fn relative_path_made_absolute() {
        let pwd = PathBuf::from("/usr");
        let parent = PathBuf::from("/");
        let sibling = PathBuf::from("/lib");
        let descendent = PathBuf::from("/usr/bin");

        check_make_abs_path(&Path::new("/usr/.."), parent);
        check_make_abs_path(&Path::new("/usr/../lib"), sibling);
        check_make_abs_path(&Path::new("/usr/../usr/bin"), descendent);
    }

    #[test]
    #[cfg(windows)]
    fn relative_path_made_absolute() {
        // need to preface Windows paths with "C:" since that's the root, by default
        let pwd = PathBuf::from("\\\\?\\C:\\Users");
        let parent = PathBuf::from("\\\\?\\C:\\");
        let sibling = PathBuf::from("\\\\?\\C:\\Windows");
        let descendent = PathBuf::from("\\\\?\\C:\\Users\\Public");

        check_make_abs_path(&pwd.join("../"), parent);
        check_make_abs_path(&pwd.join("../Windows"), sibling);
        check_make_abs_path(&pwd.join("../Users/Public"), descendent);
    }
}
