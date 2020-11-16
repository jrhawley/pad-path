mod cli;
mod path;

use cli::cli_flow;

fn main() {
    match cli_flow() {
        // if no error, do nothing
        Ok(_p) => {}
        Err(e) => eprintln!("{}", e),
    };
}
