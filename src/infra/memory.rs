use super::super::TaskRepo as DomainTaskRepo;
use super::super::UserRepo as DomainUserRepo;
use super::super::{Task, User};
use super::rand;
use std::collections::HashMap;
use std::error;

pub struct UserRepo {
    users: HashMap<String, User>,
}

impl UserRepo {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

impl DomainUserRepo for UserRepo {
    fn next_id(&self) -> Result<String, Box<dyn error::Error>> {
        Ok(rand::generate_string(50))
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn error::Error>> {
        for (_, user) in &self.users {
            if user.email() == email {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }

    fn save(&mut self, user: &User) -> Result<(), Box<dyn error::Error>> {
        self.users.insert(user.id().clone(), user.clone());
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), Box<dyn error::Error>> {
        self.users.remove(id);
        Ok(())
    }
}

pub struct TaskRepo {
    tasks: HashMap<String, Task>,
}

impl TaskRepo {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl DomainTaskRepo for TaskRepo {
    fn next_id(&self) -> Result<String, Box<dyn error::Error>> {
        Ok(rand::generate_string(70))
    }

    fn get(&self, user_id: &str) -> Result<Vec<Task>, Box<dyn error::Error>> {
        let mut tasks = Vec::new();
        for (_, task) in &self.tasks {
            if task.user_id() != user_id {
                continue;
            }

            tasks.push(task.clone());
        }

        Ok(tasks)
    }

    fn find_of_user(&self, id: &str, user_id: &str) -> Result<Option<Task>, Box<dyn error::Error>> {
        for (_, task) in &self.tasks {
            if task.id() != id || task.user_id() != user_id {
                continue;
            }

            return Ok(Some(task.clone()));
        }

        Ok(None)
    }

    fn save(&mut self, task: &Task) -> Result<(), Box<dyn error::Error>> {
        self.tasks.insert(task.id().clone(), task.clone());
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), Box<dyn error::Error>> {
        self.tasks.remove(id);
        Ok(())
    }

    fn delete_of_user(&mut self, user_id: &str) -> Result<(), Box<dyn error::Error>> {
        let ids: Vec<String> = self
            .tasks
            .values()
            .filter(|task| task.user_id() == user_id)
            .map(|task| task.id.clone())
            .collect();
        for id in ids {
            self.tasks.remove(&id);
        }

        Ok(())
    }
}
