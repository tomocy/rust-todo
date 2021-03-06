use super::super::super::{Task, User};
use super::super::controller;

pub struct Text {}

impl controller::UserRenderer for Text {
    fn render_user(&self, user: &User) {
        println!("ID: {}", user.id());
        println!("Email: {}", user.email());
    }
}

impl controller::TaskRenderer for Text {
    fn render_tasks(&self, tasks: &Vec<Task>) {
        for task in tasks {
            println!("-----");
            self.render_task(task);
        }
    }

    fn render_task(&self, task: &Task) {
        println!("ID: {}", task.id());
        println!("User ID: {}", task.user_id());
        println!("Name: {}", task.name());
        println!(
            "Status: {}",
            if task.is_completed() {
                "Completed"
            } else {
                "Not Completed"
            }
        );
    }
}

impl controller::Renderer for Text {
    fn render_message(&self, msg: &str) {
        println!("{}", msg);
    }

    fn render_error(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}
