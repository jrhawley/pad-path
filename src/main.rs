mod cli;
mod path;

use cli::cli_flow;
use crate::path::read_raw_path;

fn main() {
    match cli_flow() {
        // if no error, do nothing
        Ok(_p) => {}
        // if there is an error, print the error to STDERR and print the original path to STDOUT
        Err(e) => {
            eprintln!("{}", e);
            println!("{}", read_raw_path().unwrap().to_string_lossy());
        },
    };
}
