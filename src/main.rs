extern crate todo;

use std::process;
use todo::gateway::controller;
use todo::gateway::controller::cli;
use todo::gateway::presenter::text;
use todo::infra::file;

fn main() {
    let workspace = "./";
    let mut user_repo: Box<dyn todo::UserRepo> = Box::new(file::UserRepo::new(&workspace).unwrap());
    let mut task_repo: Box<dyn todo::TaskRepo> = Box::new(file::TaskRepo::new(&workspace).unwrap());
    let user_renderer: Box<dyn controller::UserRenderer> = Box::new(text::Text {});
    let task_renderer: Box<dyn controller::TaskRenderer> = Box::new(text::Text {});
    let mut session_manager: Box<dyn controller::SessionManager> =
        Box::new(file::SessionManager::new(&workspace).unwrap());

    let mut app = cli::App::new(
        &mut user_repo,
        &mut task_repo,
        &user_renderer,
        &task_renderer,
        &mut session_manager,
    );
    if let Err(err) = app.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
