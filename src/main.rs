use std::env::{current_dir, JoinPathsError};
use std::fs::canonicalize;
use std::path::PathBuf;
// use winreg::{enums::*, RegKey};

mod cli;
mod path;

use cli::parse_cli;
use path::{add_to_path, read_path};

fn main() -> Result<(), JoinPathsError> {
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
        let abs_dir = match indir.is_relative() {
            true => match indir.exists() {
                true => canonicalize(indir).unwrap(),
                false => {
                    let mut abs_dir = current_dir().unwrap();
                    abs_dir.push(indir);
                    abs_dir
                }
            },
            false => indir,
        };

        if !abs_dir.exists() {
            if _o.is_present("force") {
                add_to_path(abs_dir, prepend, dryrun)?
            } else {
                eprintln!(
                    "Directory does not exist. If you still want to add this, re-run with `-f/--force`."
                );
            }
        } else {
            add_to_path(abs_dir, prepend, dryrun)?
        }
    }
    Ok(())
}
