use super::super::User;

pub trait UserRenderer {
    fn render_user(&self, user: &User);
}

pub mod cli;
