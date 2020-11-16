use crate::path::{
    add_to_path, change_priority, clean_path, make_abs_path, read_old_path, read_path, revert_path,
    rm_from_path,
};
use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .takes_value(false)
                        .help("Forefully add a directory that doesn't exist"),
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
            SubCommand::with_name("undo")
                .about("Undo most recent changes to PATH")
                .visible_alias("revert")
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .takes_value(false)
                        .help("Forefully revert to OLD_PATH, regardless of if it is empty"),
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
            SubCommand::with_name("ls")
                .about("List the directories in PATH")
                .visible_alias("echo")
                .arg(
                    Arg::with_name("old")
                        .short("o")
                        .long("--old")
                        .takes_value(false)
                        .required(false)
                        .help("Show OLD_PATH instead of PATH"),
                ),
        )
        .get_matches();
    matches
}

pub fn cli_flow() -> Result<(), Error> {
    let matches = parse_cli();
    if let Some(_o) = matches.subcommand_matches("ls") {
        let show_old = _o.is_present("old");
        let vpath = match show_old {
            true => read_old_path(),
            false => read_path(),
        };
        for p in &vpath {
            println!("{}", p.display());
        }
    } else if let Some(_o) = matches.subcommand_matches("add") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let prepend = _o.is_present("prepend");
        let dryrun = _o.is_present("dryrun");

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);

        if !abs_dir.exists() {
            if _o.is_present("force") {
                return add_to_path(abs_dir, prepend, dryrun);
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Directory does not exist. If you still want to add this, re-run with `-f/--force`."
                ));
            }
        } else {
            return add_to_path(abs_dir, prepend, dryrun);
        }
    } else if let Some(_o) = matches.subcommand_matches("rm") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let dryrun = _o.is_present("dryrun");

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        return rm_from_path(abs_dir, dryrun);
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

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        return change_priority(abs_dir, -1 * (jump as i8), dryrun);
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

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        return change_priority(abs_dir, jump as i8, dryrun);
    } else if let Some(_o) = matches.subcommand_matches("clean") {
        let dryrun = _o.is_present("dryrun");
        match clean_path(dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not clean PATH. '{}'", e),
        };
    } else if let Some(_o) = matches.subcommand_matches("undo") {
        let oldpath = read_old_path();
        let force = _o.is_present("force");
        let dryrun = _o.is_present("dryrun");
        // check if OLD_PATH is empty
        if oldpath == vec![PathBuf::from("")] {
            if !force {
                return Err(Error::new(
                    ErrorKind::Other,
                    "OLD_PATH not found or is empty. If you are sure you want to revert to this, re-run with `-f/--force`.")
                );
            }
            return revert_path(dryrun);
        }
        return revert_path(dryrun);
    }
    Ok(())
}
