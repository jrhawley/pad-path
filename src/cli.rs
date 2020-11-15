use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};

pub fn parse_cli() -> ArgMatches<'static> {
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
    matches
}
