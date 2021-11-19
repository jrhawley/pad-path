//! Command line argument parsing and decision making.

use crate::path::revert_path;
use crate::path::{change_priority, clean_path, read::read_path, rm_from_path};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
    ArgMatches,
};
use std::{
    io,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

use crate::path::add::{add_to_path, AddOpt};

/// Configuration for the entire application.
///
/// This is specified by the user through the CLI.
#[derive(Debug, StructOpt)]
#[structopt(
    name = crate_name!(),
    author = crate_authors!(),
    about = crate_description!(),
    settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Option<SubCmd>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Subcommands")]
enum SubCmd {
    Add(AddOpt),
    Rm(RmOpt),
    #[structopt(about = "Increase priority for a directory", author = crate_authors!(), visible_alias = "inc", settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto])]
    Up(MvOpt),
    #[structopt(about = "Decrease priority for a directory", author = crate_authors!(), visible_aliases = &["dec", "down"], settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto])]
    Dn(MvOpt),
    #[structopt(
        about = "Remove duplicates and non-existent directories", author = crate_authors!(),
        visible_alias = "dedup", settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
    )]
    Clean,
    #[structopt(about = "List the directories in `$PATH`", author = crate_authors!(), visible_alias = "echo", settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto])]
    Ls,
    Revert(RevertOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Add a directory", author = crate_authors!(), settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto])]
struct AddOpt {
    /// Directory to add.
    #[structopt(default_value = ".")]
    dir: PathBuf,

    /// Forcefully add a directory that doesn't necessarily exist.
    #[structopt(short, long)]
    force: bool,

    /// Make this directory the highest priority by prepending it to `$PATH`
    #[structopt(short, long)]
    prepend: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Remove a directory", author = crate_authors!(), visible_alias = "del", settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto])]
struct RmOpt {
    /// Directory to remove
    #[structopt(default_value = ".")]
    dir: PathBuf,
}

#[derive(Debug, StructOpt)]
struct MvOpt {
    /// Directory to move
    #[structopt(default_value = ".")]
    dir: PathBuf,

    /// Move directory up `JUMP` spots in the `$PATH`
    #[structopt(default_value = "1")]
    jump: usize,
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Revert to a previous version of `$PATH`", author = crate_authors!(),
    visible_alias = "echo", settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
)]
struct RevertOpt {
    /// `$PATH` revision number to revert to. If not specified, reverts to the most recent version.
    #[structopt(default_value = "1")]
    revision: usize,
}

/// Execute the command issued from the command line.
///
/// Parsing of the arguments is explicitly handled by [`parse_cli`](fn.parse_cli.html).
pub fn execute_cli() -> Result<(), Error> {
    let opt = Opt::from_args();

    match &opt.cmd {
        Some(SubCmd::Ls) | None => {
            let vpath = read_path();
            for p in &vpath {
                println!("{}", p.display());
            }
        }
        Some(SubCmd::Add(add_opts)) => {
            add_opts.validate()?;
            add_to_path(&add_opts)?;
        }
        _ => {} // ("rm", Some(submatches)) => {
                //     // read command line options
                //     let indir = PathBuf::from(submatches.value_of("dir").unwrap());
                //     let dry_run = submatches.is_present("dry_run");
                //     let add_to_history = submatches.is_present("history");

                // return rm_from_path(indir, dry_run, add_to_history);
                //
                // ("up", Some(submatches)) => {
                //     // read command line options
                //     let indir = PathBuf::from(submatches.value_of("dir").unwrap());
                //     let jump = match submatches.value_of("jump").unwrap().parse::<usize>() {
                //         Ok(j) => j,
                //         Err(_) => {
                //             return Err(Error::new(
                //                 ErrorKind::InvalidInput,
                //                 "JUMP must be a whole number.",
                //             ))
                //         }
                //     };
                //     let dry_run = submatches.is_present("dry_run");
                //     let add_to_history = submatches.is_present("history");

                //     return change_priority(indir, -1 * (jump as i8), dry_run, add_to_history);
                // }
                // ("dn", Some(submatches)) => {
                //     // read command line options
                //     let indir = PathBuf::from(submatches.value_of("dir").unwrap());
                //     let jump = match submatches.value_of("jump").unwrap().parse::<usize>() {
                //         Ok(j) => j,
                //         Err(_) => {
                //             return Err(Error::new(
                //                 ErrorKind::InvalidInput,
                //                 "JUMP must be a whole number.",
                //             ))
                //         }
                //     };
                //     let dry_run = submatches.is_present("dry_run");
                //     let add_to_history = submatches.is_present("history");

                //     return change_priority(indir, jump as i8, dry_run, add_to_history);
                // }
                // ("clean", Some(submatches)) => {
                //     let dry_run = submatches.is_present("dry_run");
                //     let add_to_history = submatches.is_present("history");

                //     match clean_path(dry_run, add_to_history) {
                //         Ok(_) => {}
                //         Err(e) => eprintln!("Could not clean `$PATH`. '{}'", e),
                //     };
                // }
                // ("revert", Some(submatches)) => {
                //     let revision = match submatches
                //         .value_of("revision")
                //         .unwrap_or("1")
                //         .parse::<u128>()
                //     {
                //         Ok(j) => j,
                //         Err(_) => {
                //             return Err(Error::new(
                //                 ErrorKind::InvalidInput,
                //                 "REVISION must be a whole number >= 1.",
                //             ))
                //         }
                //     };
                //     if revision < 1 {
                //         return Err(Error::new(
                //             ErrorKind::InvalidInput,
                //             "REVISION must be a whole number >= 1.",
                //         ));
                //     }
                //     let dry_run = submatches.is_present("dry_run");
                //     let add_to_history = submatches.is_present("history");

                //     // revert to this version of the PATH
                //     match revert_path(revision, dry_run, add_to_history) {
                //         Ok(_) => {}
                //         Err(e) => eprintln!("Could not revert `$PATH`. '{}'", e),
                //     };
                // }
                // // for anything else, print the unaltered `$PATH` out of caution
                // (_, _) => {
                //     let vpath = read_path();
                //     for p in &vpath {
                //         println!("{}", p.display());
                //     }
                // }
    }
    Ok(())
}
