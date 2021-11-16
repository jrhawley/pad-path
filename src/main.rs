//! Intuitively modify your `$PATH`.

mod cli;
mod path;

use crate::cli::execute_cli;
use crate::path::read::read_raw_path;

/// Run the command line interface and print the adjusted `$PATH`.
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
