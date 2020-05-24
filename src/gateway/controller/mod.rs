use super::super::User;

pub trait UserRenderer: Renderer {
    fn render_user(&self, user: &User);
}

pub trait Renderer {
    fn render_message(&self, msg: &str);
    fn render_error(&self, msg: &str);
}

pub mod cli;
