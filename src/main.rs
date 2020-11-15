use std::path::PathBuf;
// use winreg::{enums::*, RegKey};

mod cli;
mod path;

use cli::parse_cli;
use path::{add_to_path, change_priority, clean_path, make_abs_path, read_path, rm_from_path};

fn main() {
    let matches = parse_cli();
    if let Some(_o) = matches.subcommand_matches("ls") {
        let vpath = read_path();
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
                match add_to_path(abs_dir, prepend, dryrun) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Count not add to PATH. '{}'", e),
                };
            } else {
                eprintln!(
                    "Directory does not exist. If you still want to add this, re-run with `-f/--force`."
                );
            }
        } else {
            match add_to_path(abs_dir, prepend, dryrun) {
                Ok(_) => {}
                Err(e) => eprintln!("Count not add to PATH. '{}'", e),
            };
        }
    } else if let Some(_o) = matches.subcommand_matches("rm") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let dryrun = _o.is_present("dryrun");

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        match rm_from_path(abs_dir, dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Count remove from PATH. '{}'", e),
        };
    } else if let Some(_o) = matches.subcommand_matches("up") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let jump = match _o.value_of("jump").unwrap().parse::<usize>() {
            Ok(j) => j,
            Err(_) => panic!("JUMP must be an integer."),
        };
        let dryrun = _o.is_present("dryrun");

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        match change_priority(abs_dir, -1 * (jump as i8), dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not reorder PATH. '{}'", e),
        };
    } else if let Some(_o) = matches.subcommand_matches("dn") {
        // read command line options
        let indir = PathBuf::from(_o.value_of("dir").unwrap());
        let jump = match _o.value_of("jump").unwrap().parse::<usize>() {
            Ok(j) => j,
            Err(_) => panic!("JUMP must be an integer."),
        };
        let dryrun = _o.is_present("dryrun");

        // convert to absolute directory
        let abs_dir = make_abs_path(&indir);
        match change_priority(abs_dir, jump as i8, dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not reorder PATH. '{}'", e),
        };
    } else if let Some(_o) = matches.subcommand_matches("clean") {
        let dryrun = _o.is_present("dryrun");
        match clean_path(dryrun) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not clean PATH. '{}'", e),
        };
    }
}
