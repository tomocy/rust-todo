use std::collections::HashMap;

pub struct UserRepo {
    users: HashMap<String, super::super::User>,
}

impl UserRepo {
    pub fn new() -> UserRepo {
        UserRepo {
            users: HashMap::new(),
        }
    }
}

impl super::super::UserRepo for UserRepo {
    fn next_id(&self) -> Result<String, String> {
        Ok(super::rand::generate_string(50))
    }

    fn save(&mut self, user: &super::super::User) -> Result<(), String> {
        self.users.insert(user.id().clone(), user.clone());
        Ok(())
    }}
