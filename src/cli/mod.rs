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

    match &opt.cmd {
        Some(SubCmd::Ls) | None => {
            let vpath = read_path();
            for p in &vpath {
                println!("{}", p.display());
            }
        }
        Some(SubCmd::Add(add_opts)) => {
            add_opts.validate()?;
            add_to_path(add_opts)?;
        }
        Some(SubCmd::Rm(rm_opts)) => {
            rm_opts.validate()?;
            rm_from_path(rm_opts)?;
        }
        Some(SubCmd::Clean(clean_opts)) => {
            clean_opts.validate()?;
            clean_path(clean_opts)?;
        }
        Some(SubCmd::Up(up_opts)) => {
            up_opts.validate()?;
            increase_priority(up_opts)?;
        }
        Some(SubCmd::Dn(dn_opts)) => {
            dn_opts.validate()?;
            decrease_priority(dn_opts)?;
        }
        Some(SubCmd::Revert(rev_opts)) => {
            rev_opts.validate()?;
            revert_path(rev_opts)?;
        }
    }
    Ok(())
}
