extern crate todo;

use std::process;
use todo::gateway::controller;
use todo::gateway::controller::cli;
use todo::gateway::presenter::text;
use todo::infra::file;

fn main() {
    let mut repo: Box<dyn todo::UserRepo> = Box::new(file::UserRepo::new("./"));
    let renderer: Box<dyn controller::UserRenderer> = Box::new(text::Text {});
    let mut app = cli::App::new(&mut repo, &renderer);
    if let Err(err) = app.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
