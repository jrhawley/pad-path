//! Add a directory or multiple directories to the `$PATH`.

use clap::{crate_authors, AppSettings};
use std::collections::HashSet;
use std::env::join_paths;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use super::clean_dir_name;
use super::read::read_path;
use super::write::replace_path;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Add a directory",
    author = crate_authors!(),
    settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
)]
pub struct AddOpt {
    /// Directory(ies) to add
    #[structopt(default_value = ".", name = "dir")]
    dirs: Vec<PathBuf>,

    /// Forcefully add a directory that doesn't necessarily exist.
    #[structopt(short, long)]
    force: bool,

    /// Make this directory the highest priority by prepending it to `$PATH`
    #[structopt(short, long)]
    prepend: bool,

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

impl AddOpt {
    /// Validate the options
    pub fn validate(&self) -> io::Result<()> {
        // check if directory(ies) exist if `force` is not specified
        if !self.force {
            for d in &self.dirs {
                if !d.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Directory `{}`  does not exist. `Please double check the directories you intend to add.",
                            d.display()
                        )
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Add the given directory to the `$PATH` environment variable
pub fn add_to_path(opts: &AddOpt) -> io::Result<()> {
    // read the path, clean each entry, and convert into Vec<PathBuf>
    let mut current_path: Vec<PathBuf> = read_path();
    let mut cleaned_dirs: Vec<PathBuf> = opts.dirs.iter().map(|d| clean_dir_name(d)).collect();

    // check that the directories to be added don't already exist in the PATH
    let _current_dirs: HashSet<PathBuf> = current_path.iter().map(|d| d.clone()).collect();
    let _new_dirs: HashSet<PathBuf> = cleaned_dirs.iter().map(|d| d.clone()).collect();
    let _intersecting_dirs: Vec<&Path> = _current_dirs
        .intersection(&_new_dirs)
        .into_iter()
        .map(|d| d.as_path())
        .collect();
    if _intersecting_dirs.len() > 0 {
        return Err(
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "Directory `{}` already exists in `$PATH`. Use `pad up/dn` to change priority of this directory. No changes made.",
                    _intersecting_dirs[0].display()
                )
            )
        );
    }
    let newpath = match opts.prepend {
        true => {
            cleaned_dirs.append(&mut current_path);
            join_paths(cleaned_dirs).unwrap()
        }
        false => {
            current_path.append(&mut cleaned_dirs);
            join_paths(current_path).unwrap()
        }
    };
    replace_path(newpath, opts.dry_run, opts.history)
}
