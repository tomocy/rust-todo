use super::super::User;

pub trait UserRenderer: Renderer {
    fn render_user(&self, user: &User);
}

pub trait Renderer {
    fn render_message(&self, msg: &str);
    fn render_error(&self, msg: &str);
}

pub trait SessionManager {
    fn push_authenticated_user_id(&mut self, user_id: &str) -> Result<(), String>;
    fn pop_authenticated_user_id(&self) -> Result<Option<String>, String>;
}

pub mod cli;
