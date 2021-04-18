use crate::path::{add_to_path, change_priority, clean_path, read_path, rm_from_path};
use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
use std::path::PathBuf;
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

mod path;
use crate::path::read_raw_path;

/// Parse command line arguments and return the parsed ArgMatches object
fn parse_cli() -> ArgMatches<'static> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
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
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
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
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
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
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
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
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Remove duplicates and non-existent directories")
                .visible_alias("dedup")
                .arg(
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
                        .takes_value(false)
                        .help("Only show the changes to PATH, don't actually make changes to PATH"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("List the directories in PATH")
                .visible_alias("echo"),
        )
        .get_matches();
    matches
}

/// Execute the command issued from the command line.
/// Parsing of the arguments is explicitly done by `parse_cli`.
fn execute_cli() -> Result<(), Error> {
    let matches = parse_cli();
    if let Some(_o) = matches.subcommand_matches("ls") {
        let vpath = read_path();
        for p in &vpath {
            println!("{}", p.display());
        }
    } else if let Some(_o) = matches.subcommand_matches("add") {
        // read command line options
        let indir = _o.values_of("dir");
        if indir.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid input. Please double check the directories you intend to add.",
            ));
        }
        let mut indirs: Vec<PathBuf> = indir.unwrap().map(|d| PathBuf::from(d)).collect();
        let prepend = _o.is_present("prepend");
        let dryrun = _o.is_present("dryrun");

        // check for the existence of directories to be added
        let missing_dirs: Vec<&Path> = indirs
            .iter()
            .filter(|&d| !d.exists())
            .map(|d| d.as_path())
            .collect();
        let _all_dirs_exist = missing_dirs.len() == 0;

        if !_all_dirs_exist {
            // proceed if `--force` is supplied
            if _o.is_present("force") {
                return add_to_path(&mut indirs, prepend, dryrun);
            } else {
                // don't proceed, tell the user to try again
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Directory `{}` does not exist. If you still want to add this, re-run with `-f/--force`.", missing_dirs[0].display())
                ));
            }
        } else {
            return add_to_path(&mut indirs, prepend, dryrun);
        }
    } else if let Some(_o) = matches.subcommand_matches("rm") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let dryrun = _o.is_present("dryrun");
        return rm_from_path(indir, dryrun);
    } else if let Some(_o) = matches.subcommand_matches("up") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let jump = match _o.value_of("jump").unwrap().parse::<usize>() {
            Ok(j) => j,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "JUMP must be an integer.",
                ))
            }
        };
        let dryrun = _o.is_present("dryrun");

        return change_priority(indir, -1 * (jump as i8), dryrun);
    } else if let Some(_o) = matches.subcommand_matches("dn") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let jump = match _o.value_of("jump").unwrap().parse::<usize>() {
            Ok(j) => j,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "JUMP must be an integer.",
                ))
            }
        };
        let dryrun = _o.is_present("dryrun");

        return change_priority(indir, jump as i8, dryrun);
    } else if let Some(_o) = matches.subcommand_matches("clean") {
        let dryrun = _o.is_present("dryrun");
        match clean_path(dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not clean PATH. '{}'", e),
        };
    }
    Ok(())
}

fn main() {
    match execute_cli() {
        // if no error, do nothing
        Ok(_p) => {}
        // if there is an error, print the error to STDERR and print the original path to STDOUT
        Err(e) => {
            eprintln!("{}", e);
            println!("{}", read_raw_path().unwrap().to_string_lossy());
        }
    };
}
