use std::env;
use std::env::{current_dir, join_paths, split_paths, JoinPathsError};
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
// use winreg::{enums::*, RegKey};

mod cli;

use cli::parse_cli;

fn read_raw_path() -> String {
    match env::var_os("PATH") {
        Some(p_str) => String::from(p_str.to_str().unwrap()),
        None => String::new(),
    }
}

fn read_path() -> Vec<PathBuf> {
    let path_str = read_raw_path();
    split_paths(&path_str).into_iter().collect()
}

fn replace_windows_path(path_str: &str) {
    // need to use Registry Editor to edit environment variables on windows
    //     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    //     let (env, _) = hkcu.create_subkey("Environment").unwrap();
}

fn replace_nix_path(path_str: &str) {}

fn add_to_path(dir: PathBuf, prepend: bool, dryrun: bool) -> Result<(), JoinPathsError> {
    // read the path and convert into Vec<&PathBuf>
    let mut current_path: Vec<PathBuf> = read_path();
    let newpath = match prepend {
        true => {
            let mut all_paths = vec![dir];
            all_paths.append(&mut current_path);
            join_paths(all_paths)?
        }
        false => {
            let mut all_paths = vec![dir];
            current_path.append(&mut all_paths);
            join_paths(current_path)?
        }
    };
    if dryrun {
        println!("PATH before modifcation:\n\t{}", read_raw_path());
        println!("PATH after modifcation:\n\t{}", newpath.to_str().unwrap());
    } else {
        // env::set_var("PATH", newpath);
    }
    Ok(())
}

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
