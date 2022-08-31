//! Remove a directory or multiple directories from the `$PATH`.

use super::read::{combine_path_like, read_path};
use super::write::replace_path;
use clap::{crate_authors, Parser};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    about = "Remove a directory",
    author = crate_authors!(),
    visible_alias = "del",
)]
pub struct RmOpt {
    /// Directory(ies) to remove
    #[clap(default_value = ".")]
    dir: PathBuf,

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

impl RmOpt {
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        // check if directory exists
        if !self.dir.exists() {
            let err_nonexistent = io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Directory `{}`  does not exist. `Please double check the directory you intend to remove.",
                    self.dir.display()
                )
            );

            if !self.quiet {
                eprintln!("{}", err_nonexistent);
            }

            return Err(err_nonexistent);
        }

        // check the directory to remove exists in `$PATH`
        let current_path = read_path();
        if !current_path.iter().any(|x| *x == self.dir) {
            let err_not_found = io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Directory `{}` not found in `$PATH`. No changes made.",
                    self.dir.display()
                ),
            );

            if !self.quiet {
                eprintln!("{}", err_not_found);
            }

            return Err(err_not_found);
        }

        Ok(())
    }
}

/// Remove the given directory to the `$PATH` environment variable
pub fn rm_from_path(opts: &RmOpt) -> io::Result<()> {
    let current_path = read_path();
    let i = current_path.iter().position(|x| *x == opts.dir).unwrap();

    let mut vpath = current_path;
    vpath.remove(i);
    let newpath = combine_path_like(vpath)?;
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
