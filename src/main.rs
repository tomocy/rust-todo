extern crate todo;

use std::process;
use todo::gateway::controller::cli;

fn main() {
    let app = cli::App {};
    if let Err(err) = app.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
