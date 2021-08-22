//! Command line argument parsing and decision making.

use crate::path::{add_to_path, change_priority, clean_path, read::read_path, rm_from_path};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches,
    SubCommand,
};
use std::path::PathBuf;
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

/// Parse command line arguments and return the parsed ArgMatches object
fn parse_cli() -> ArgMatches<'static> {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a directory")
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to add")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .takes_value(false)
                        .help("Forcefully add a directory that doesn't exist"),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .short("n")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                )
                .arg(
                    Arg::with_name("prepend")
                        .short("p")
                        .long("prepend")
                        .help("Make this directory the highest priority by prepending it to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a directory")
                .visible_alias("del")
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to remove")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .short("n")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                )
                .arg(
                    Arg::with_name("prepend")
                        .short("p")
                        .long("prepend")
                        .help("Make this directory the highest priority by prepending it to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("up")
                .about("Increase priority for a directory")
                .visible_alias("inc")
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to move")
                        .required(true)
                        .takes_value(true)
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("jump")
                        .value_name("JUMP")
                        .help("Move this directory up `JUMP` spots in the PATH.")
                        .required(true)
                        .takes_value(true)
                        .default_value("1"),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .short("n")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("dn")
                .about("Decrease priority for a directory")
                .visible_aliases(&["down", "dec"])
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to move down")
                        .required(true)
                        .takes_value(true)
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("jump")
                        .value_name("JUMP")
                        .help("Move this directory down `JUMP` spots in the PATH.")
                        .required(true)
                        .takes_value(true)
                        .default_value("1"),
                )
                .arg(
                    Arg::with_name("dry_run")
                        .short("n")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Remove duplicates and non-existent directories")
                .visible_alias("dedup")
                .arg(
                    Arg::with_name("dry_run")
                        .short("n")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("List the directories in PATH")
                .visible_alias("echo"),
        )
        .subcommand(
            SubCommand::with_name("revert")
                .about("Revert to a previous version of PATH")
                .visible_alias("undo")
                .arg(
                    Arg::with_name("revision")
                        .help("PATH revision number to revert to. If not specified, reverts to the most recent version, by default.")
                        .short("r")
                        .long("revision")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name("list")
                        .help("List all previous recorded revisions of PATH")
                        .short("l")
                        .long("list")
                        .takes_value(false)
                        .required(true),
                ),
        )
        .get_matches();
    matches
}

/// Execute the command issued from the command line.
/// Parsing of the arguments is explicitly done by `parse_cli`.
pub fn execute_cli() -> Result<(), Error> {
    let matches = parse_cli();
    match matches.subcommand() {
        ("ls", _) => {
            let vpath = read_path();
            for p in &vpath {
                println!("{}", p.display());
            }
        }
        ("add", Some(submatches)) => {
            // read command line options
            let indir = submatches.values_of("dir");
            if indir.is_none() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid input. Please double check the directories you intend to add.",
                ));
            }
            let mut indirs: Vec<PathBuf> = indir.unwrap().map(|d| PathBuf::from(d)).collect();
            let prepend = submatches.is_present("prepend");
            let dry_run = submatches.is_present("dry_run");

            // check for the existence of directories to be added
            let missing_dirs: Vec<&Path> = indirs
                .iter()
                .filter(|&d| !d.exists())
                .map(|d| d.as_path())
                .collect();
            let _all_dirs_exist = missing_dirs.len() == 0;

            if !_all_dirs_exist {
                // proceed if `--force` is supplied
                if submatches.is_present("force") {
                    return add_to_path(&mut indirs, prepend, dry_run);
                } else {
                    // don't proceed, tell the user to try again
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Directory `{}` does not exist. If you still want to add this, re-run with `-f/--force`.", missing_dirs[0].display())
                    ));
                }
            } else {
                return add_to_path(&mut indirs, prepend, dry_run);
            }
        }
        ("rm", Some(submatches)) => {
            // read command line options
            let indir = PathBuf::from(submatches.value_of("dir").unwrap());
            let dry_run = submatches.is_present("dry_run");
            return rm_from_path(indir, dry_run);
        }
        ("up", Some(submatches)) => {
            // read command line options
            let indir = PathBuf::from(submatches.value_of("dir").unwrap());
            let jump = match submatches.value_of("jump").unwrap().parse::<usize>() {
                Ok(j) => j,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "JUMP must be an integer.",
                    ))
                }
            };
            let dry_run = submatches.is_present("dry_run");

            return change_priority(indir, -1 * (jump as i8), dry_run);
        }
        ("dn", Some(submatches)) => {
            // read command line options
            let indir = PathBuf::from(submatches.value_of("dir").unwrap());
            let jump = match submatches.value_of("jump").unwrap().parse::<usize>() {
                Ok(j) => j,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "JUMP must be an integer.",
                    ))
                }
            };
            let dry_run = submatches.is_present("dry_run");

            return change_priority(indir, jump as i8, dry_run);
        }
        ("clean", Some(submatches)) => {
            let dry_run = submatches.is_present("dry_run");
            match clean_path(dry_run) {
                Ok(_) => {}
                Err(e) => eprintln!("Could not clean PATH. '{}'", e),
            };
        }
        // for anything else, print the unaltered PATH out of caution
        (_, _) => {
            let vpath = read_path();
            for p in &vpath {
                println!("{}", p.display());
            }
        }
    }
    Ok(())
}
