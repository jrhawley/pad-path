//! Remove a directory or multiple directories from the `$PATH`.

use clap::{crate_authors, AppSettings};
use std::env::join_paths;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

use super::read::read_path;
use super::write::replace_path;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Remove a directory",
    author = crate_authors!(),
    visible_alias = "del",
    settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
)]
pub struct RmOpt {
    /// Directory(ies) to remove
    #[structopt(default_value = ".")]
    dir: PathBuf,

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

impl RmOpt {
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        // check if directory exists
        if !self.dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Directory `{}`  does not exist. `Please double check the directory you intend to remove.",
                    self.dir.display()
                )
            ));
        }

        Ok(())
    }
}

/// Remove the given directory to the `$PATH` environment variable
pub fn rm_from_path(opts: &RmOpt) -> io::Result<()> {
    let current_path = read_path();
    let idx = current_path.iter().position(|x| *x == opts.dir);
    // if the directory is found within PATH
    if let Some(i) = idx {
        let mut vpath = current_path.clone();
        vpath.remove(i);
        let newpath = join_paths(vpath).unwrap();
        match replace_path(newpath, opts.dry_run, opts.history, opts.quiet) {
            Ok(()) => Ok(()),
            Err(e) => {
                if !opts.quiet {
                    eprintln!("{}", e);
                }

                Err(e)
            }
        }
    } else {
        let err = io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Directory `{}` not found in `$PATH`. No changes made.",
                opts.dir.display()
            ),
        );

        if !opts.quiet {
            eprintln!("{}", err);
        }

        Err(err)
    }
}
