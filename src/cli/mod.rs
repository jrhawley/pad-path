//! Command line argument parsing and decision making.

use clap::{crate_authors, crate_description, crate_name, AppSettings};
use std::io;
use structopt::StructOpt;

use crate::path::{
    add::{add_to_path, AddOpt},
    clean::{clean_path, CleanOpt},
    priority::{decrease_priority, increase_priority, MvOpt},
    read::read_path,
    remove::{rm_from_path, RmOpt},
    revert::{revert_path, RevertOpt},
};

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
    #[structopt(
        about = "Increase priority for a directory",
        author = crate_authors!(),
        visible_alias = "inc",
        settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
    )]
    Up(MvOpt),
    #[structopt(
        about = "Decrease priority for a directory",
        author = crate_authors!(),
        visible_aliases = &["dec", "down"],
        settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
    )]
    Dn(MvOpt),
    Clean(CleanOpt),
    #[structopt(
        about = "List the directories in `$PATH`",
        author = crate_authors!(),
        visible_alias = "echo",
        settings = &[AppSettings::ColoredHelp, AppSettings::ColorAuto]
    )]
    Ls,
    Revert(RevertOpt),
}

/// Execute the command issued from the command line.
///
/// Parsing of the arguments is explicitly handled by [`parse_cli`](fn.parse_cli.html).
pub fn execute_cli() -> io::Result<()> {
    let opt = Opt::from_args();
    println!("{:#?}", &opt);

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
        Some(SubCmd::Rm(rm_opts)) => {
            rm_opts.validate()?;
            rm_from_path(&rm_opts)?;
        }
        Some(SubCmd::Clean(clean_opts)) => {
            clean_opts.validate()?;
            match clean_path(&clean_opts) {
                Ok(_) => {}
                Err(e) => eprintln!("Could not clean `$PATH`. '{}'", e),
            }
        }
        Some(SubCmd::Up(up_opts)) => {
            up_opts.validate()?;
            increase_priority(&up_opts)?;
        }
        Some(SubCmd::Dn(dn_opts)) => {
            dn_opts.validate()?;
            decrease_priority(&dn_opts)?;
        }
        _ => {} // ("revert", Some(submatches)) => {
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
    }
    Ok(())
}
