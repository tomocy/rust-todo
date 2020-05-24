extern crate todo;

use std::process;
use todo::gateway::controller::cli;
use todo::infra::memory;

fn main() {
    let mut repo: Box<dyn todo::UserRepo> = Box::new(memory::UserRepo::new());
    let mut app = cli::App::new(&mut repo);
    if let Err(err) = app.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
