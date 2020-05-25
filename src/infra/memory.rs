use super::super::TaskRepo as DomainTaskRepo;
use super::super::UserRepo as DomainUserRepo;
use super::super::{Task, User};
use std::collections::HashMap;

pub struct UserRepo {
    users: HashMap<String, User>,
}

impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            users: HashMap::new(),
        }
    }
}

impl DomainUserRepo for UserRepo {
    fn next_id(&self) -> Result<String, String> {
        Ok(super::rand::generate_string(50))
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        for (_, user) in &self.users {
            if user.email() == email {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }

    fn save(&mut self, user: &User) -> Result<(), String> {
        self.users.insert(user.id().clone(), user.clone());
        Ok(())
    }
}

pub struct TaskRepo {
    tasks: HashMap<String, Task>,
}

impl TaskRepo {
    pub fn new() -> Self {
        TaskRepo {
            tasks: HashMap::new(),
        }
    }
}

impl DomainTaskRepo for TaskRepo {
    fn next_id(&self) -> Result<String, String> {
        Ok(super::rand::generate_string(70))
    }

    fn save(&mut self, task: &Task) -> Result<(), String> {
        self.tasks.insert(task.id().clone(), task.clone());
        Ok(())
    }
}
