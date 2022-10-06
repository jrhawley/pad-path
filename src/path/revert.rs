//! Revert `$PATH` to a previous value.

use super::{history::get_nth_last_revision, write::replace_path};
use clap::{crate_authors, Parser};
use std::io;

#[derive(Debug, Parser)]
#[clap(
    about = "Revert to a previous version of `$PATH`",
    author = crate_authors!(),
)]
pub struct RevertOpt {
    /// `$PATH` revision number to revert to.
    /// If not specified, reverts to the most recent version.
    /// Must be a positive whole number.
    #[clap(default_value = "1")]
    revision: u128,

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

impl RevertOpt {
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        Ok(())
    }
}

/// Revert to an earlier `$PATH`
///
/// This makes use of the `.path_history` file
pub fn revert_path(opts: &RevertOpt) -> io::Result<()> {
    // look up an old `$PATH` from the path history
    let newpath = match get_nth_last_revision(opts.revision) {
        Ok(s) => s,
        Err(e) => {
            if !opts.quiet {
                eprintln!("{}", e);
            }

            return Err(e);
        }
    };

    // replace the current path with the revised one
    match replace_path(newpath, opts.dry_run, opts.history, opts.quiet) {
        Ok(()) => Ok(()),
        Err(e) => {
            if !opts.quiet {
                eprintln!("Could not revert `$PATH`. {}. No changes made.", e);
            }

            Err(e)
        }
    }
}
