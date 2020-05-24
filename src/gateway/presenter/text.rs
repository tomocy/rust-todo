use super::super::super::User;
use super::super::controller;

pub struct Text {}

impl controller::UserRenderer for Text {
    fn render_user(&self, user: &User) {
        println!("ID: {}", user.id());
        println!("Email: {}", user.email());
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
