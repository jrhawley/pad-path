use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::env;
use std::env::{join_paths, split_paths, JoinPathsError, VarError};
use std::path::{Path, PathBuf};
// use winreg::{enums::*, RegKey};

fn read_path() -> Result<Vec<PathBuf>, VarError> {
    let path_str = env::var_os("PATH");
    match path_str {
        Some(p_str) => Ok(split_paths(&p_str).into_iter().collect()),
        None => Err(VarError::NotPresent),
    }
}

fn replace_windows_path(path_str: &str) {
    // need to use Registry Editor to edit environment variables on windows
    //     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    //     let (env, _) = hkcu.create_subkey("Environment").unwrap();
}

fn replace_nix_path(path_str: &str) {}

fn add_to_path(dir: &PathBuf, prepend: bool) -> Result<(), JoinPathsError> {
    let mut current_path = read_path().unwrap_or_default();
    let newpath = match prepend {
        true => {
            let mut all_paths = vec![*dir];
            all_paths.append(&mut current_path);
            join_paths(all_paths)?
        }
        false => {
            let mut all_paths = vec![*dir];
            current_path.append(&mut all_paths);
            join_paths(current_path)?
        }
    };
    env::set_var("PATH", newpath);
    Ok(())
}

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
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
                        .help("Directory to add")
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
            SubCommand::with_name("pri")
                .about("Change the priority for a directory")
                .visible_alias("priority")
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to move up or down")
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
                    Arg::with_name("jump")
                        .short("j")
                        .long("jump-number")
                        .value_name("JUMP")
                        .help("Move this directory up or down `JUMP` spots in the PATH")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("List the directories in PATH")
                .visible_alias("echo"),
        )
        .get_matches();

    if let Some(_o) = matches.subcommand_matches("ls") {
        let vpath = read_path();
        for p in &vpath.unwrap() {
            println!("{}", p.display());
        }
    } else if let Some(_o) = matches.subcommand_matches("add") {
        // verify input directory and convert to absolute path
        let dir = PathBuf::from(_o.value_of("dir").unwrap());
        if !dir.exists() {
            if matches.is_present("force") {
                add_to_path(&dir, matches.is_present("prepend"));
            } else {
                eprintln!(
                    "Directory does not exist. If you still want to add this, re-run with `--force`."
                )
            }
        }
    }
}
