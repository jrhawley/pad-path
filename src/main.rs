use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::env;
use std::env::consts::OS;
use std::env::VarError;

fn read_path() -> Result<Vec<String>, VarError> {
    let path_sep: &str = match OS {
        "windows" => ";",
        _ => ":",
    };
    let path_str = env::var("PATH")?;
    let split_path: Vec<&str> = path_str.split(path_sep).collect();
    let vpath: Vec<String> = split_path.iter().map(|p| String::from(*p)).collect();
    Ok(vpath)
}

fn main() {
    let matches =
        App::new(crate_name!())
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
                        Arg::with_name("prepend").short("p").long("prepend").help(
                            "Make this directory the highest priority by prepending it to PATH",
                        ),
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
                        Arg::with_name("prepend").short("p").long("prepend").help(
                            "Make this directory the highest priority by prepending it to PATH",
                        ),
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
            println!("{}", p);
        }
    }
}
