//! Add a directory or multiple directories to the `$PATH`.

use clap::{crate_authors, AppSettings};
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use super::clean::clean_dirs_names;
use super::read::{read_path, combine_path_like};
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
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        if self.force {
            return Ok(());
        }

        // check if directory(ies) exist
        for d in &self.dirs {
            if !d.exists() {
                let err_nonexistent = io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Directory `{}`  does not exist. Please double check the directories you intend to add.",
                        d.display()
                    )
                );

                if !self.quiet {
                    eprintln!("{}", err_nonexistent);
                }

                return Err(err_nonexistent);
            }
        }

        // check that the directories to be added don't already exist in the PATH
        let cleaned_dirs: Vec<PathBuf> = clean_dirs_names(&self.dirs);
        let _current_dirs: HashSet<PathBuf> = read_path().iter().cloned().collect();
        let _new_dirs: HashSet<PathBuf> = cleaned_dirs.iter().cloned().collect();

        let _intersecting_dirs: Vec<&Path> = _current_dirs
            .intersection(&_new_dirs)
            .into_iter()
            .map(|d| d.as_path())
            .collect();

            if !_intersecting_dirs.is_empty() {
            let err_duplicated = io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "Directory `{}` already exists in `$PATH`. Use `pad up/dn` to change priority of this directory, or `pad add -f` to force it. No changes made.",
                    _intersecting_dirs[0].display()
                )
            );

            if !self.quiet {
                eprintln!("{}", err_duplicated);
            }

            return Err(err_duplicated);
        }

        Ok(())
    }
}

/// Add the given directory to the `$PATH` environment variable
pub fn add_to_path(opts: &AddOpt) -> io::Result<()> {
    // read the path, clean each entry, and convert into Vec<PathBuf>
    let mut current_path: Vec<PathBuf> = read_path();
    let mut cleaned_dirs: Vec<PathBuf> = clean_dirs_names(&opts.dirs);

    let newpath = match opts.prepend {
        true => {
            cleaned_dirs.append(&mut current_path);
            combine_path_like(cleaned_dirs)
        }
        false => {
            current_path.append(&mut cleaned_dirs);
            combine_path_like(current_path)
        }
    }?;
    match replace_path(newpath, opts.dry_run, opts.history, opts.quiet) {
        Ok(()) => Ok(()),
        Err(e) => {
            if !opts.quiet {
                eprintln!("{}", e);
            }
            Err(e)
        }
    }
}
