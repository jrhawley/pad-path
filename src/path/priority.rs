//! Change the priority of a directory in `$PATH`.

use std::{cmp::min, io, path::PathBuf};
use structopt::StructOpt;

use super::{read::{read_path, combine_path_like}, write::replace_path};

#[derive(Debug, StructOpt)]
pub struct MvOpt {
    /// Directory to move
    #[structopt(default_value = ".")]
    dir: PathBuf,

    /// Move directory up `JUMP` spots in the `$PATH`
    #[structopt(default_value = "1")]
    jump: usize,

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

impl MvOpt {
    /// Validate options
    pub fn validate(&self) -> io::Result<()> {
        Ok(())
    }
}

/// Change the priority of a directory by moving it earlier or later in `$PATH`.
///
/// A negative value for `direction_factor` means the directory is increasing
/// in priority (a smaller index value).
/// A positive value for `direction_factor` means the directory is decreasing
/// in priority (a larger index value).
fn change_priority(opts: &MvOpt, direction_factor: i8) -> io::Result<()> {
    let current_path = read_path();
    let idx = current_path.iter().position(|x| *x == opts.dir);
    // if the directory is found within `$PATH`
    if let Some(i) = idx {
        // calculate the new position for `dir`, and ensure that it is within the appropriate bounds
        let i_signed = i as i8;

        let signed_jump = direction_factor * (opts.jump as i8);
        let signed_new_idx = i_signed + signed_jump;
        let new_idx: usize;
        if signed_new_idx < 0 {
            new_idx = 0;
        } else {
            new_idx = min((signed_new_idx) as usize, current_path.len() - 1);
        }

        // if moving to a higher priority
        let vpath: Vec<PathBuf> = match signed_jump.cmp(&0) {
            std::cmp::Ordering::Less => {
                // get the first few elements of PATH
                let mut _vpath: Vec<PathBuf> = (0..new_idx)
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect();
                // move `dir` into the next position
                _vpath.push(opts.dir.clone());
                // add remaining elements
                _vpath.append(
                    &mut (new_idx..i)
                        .into_iter()
                        .map(|j| current_path[j].clone())
                        .collect(),
                );
                _vpath.append(
                    &mut ((i + 1)..current_path.len())
                        .into_iter()
                        .map(|j| current_path[j].clone())
                        .collect(),
                );

                _vpath
            },
            std::cmp::Ordering::Equal => {
                current_path
            },
            std::cmp::Ordering::Greater => {
                // get the first few elements of PATH
                let mut _vpath: Vec<PathBuf> = (0..i)
                    .into_iter()
                    .map(|j| current_path[j].clone())
                    .collect();
                _vpath.append(
                    &mut ((i + 1)..(new_idx + 1))
                        .into_iter()
                        .map(|j| current_path[j].clone())
                        .collect(),
                );
                // move `dir` into the next position
                _vpath.push(opts.dir.clone());
                // add remaining elements
                _vpath.append(
                    &mut ((new_idx + 1)..current_path.len())
                        .into_iter()
                        .map(|j| current_path[j].clone())
                        .collect(),
                );

                _vpath
            },
        };

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

/// Increase the priority of a directory in `$PATH`.
pub fn increase_priority(opts: &MvOpt) -> io::Result<()> {
    change_priority(opts, -1)
}

/// Decrease the priority of a directory in `$PATH`.
pub fn decrease_priority(opts: &MvOpt) -> io::Result<()> {
    change_priority(opts, 1)
}
