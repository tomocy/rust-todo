use super::*;

pub struct CreateUser<'a> {
    repo: &'a mut Box<dyn UserRepo>,
}

impl<'a> CreateUser<'a> {
    pub fn new(repo: &'a mut Box<dyn UserRepo>) -> Self {
        Self { repo: repo }
    }

    pub fn invoke(&mut self, email: &str, password: &str) -> Result<User, String> {
        let id = self.repo.next_id()?;
        let password = Hash::new(password)?;
        let user = User::new(&id, email, &password)?;

        self.repo.save(&user)?;

        Ok(user)
    }
}

pub struct AuthenticateUser<'a> {
    repo: &'a Box<dyn UserRepo>,
}

impl<'a> AuthenticateUser<'a> {
    pub fn new(repo: &'a Box<dyn UserRepo>) -> Self {
        Self { repo }
    }

    pub fn invoke(&self, email: &str, password: &str) -> Result<Option<User>, String> {
        let user = self.repo.find_by_email(email)?;
        match user {
            Some(user) if user.password().verify(password)? => Ok(Some(user)),
            _ => Ok(None),
        }
    }
}

pub struct DeleteUser<'a> {
    user_repo: &'a mut Box<dyn UserRepo>,
    task_repo: &'a mut Box<dyn TaskRepo>,
}

impl<'a> DeleteUser<'a> {
    pub fn new(user_repo: &'a mut Box<dyn UserRepo>, task_repo: &'a mut Box<dyn TaskRepo>) -> Self {
        Self {
            user_repo,
            task_repo,
        }
    }

    pub fn invoke(&mut self, id: &str) -> Result<(), String> {
        self.task_repo.delete_of_user(id)?;
        self.user_repo.delete(id)
    }
}

pub struct GetTasks<'a> {
    repo: &'a Box<dyn TaskRepo>,
}

impl<'a> GetTasks<'a> {
    pub fn new(repo: &'a Box<dyn TaskRepo>) -> Self {
        Self { repo }
    }

    pub fn invoke(&self, user_id: &str) -> Result<Vec<Task>, String> {
        self.repo.get(user_id)
    }
}

pub struct CreateTask<'a> {
    repo: &'a mut Box<dyn TaskRepo>,
}

impl<'a> CreateTask<'a> {
    pub fn new(repo: &'a mut Box<dyn TaskRepo>) -> Self {
        Self { repo }
    }

    pub fn invoke(&mut self, user_id: &str, name: &str) -> Result<Task, String> {
        let id = self.repo.next_id()?;
        let task = Task::new(&id, user_id, name)?;

        self.repo.save(&task)?;

        Ok(task)
    }
}

pub struct CompleteTask<'a> {
    repo: &'a mut Box<dyn TaskRepo>,
}

impl<'a> CompleteTask<'a> {
    pub fn new(repo: &'a mut Box<dyn TaskRepo>) -> Self {
        Self { repo }
    }

    pub fn invoke(&mut self, id: &str, user_id: &str) -> Result<Task, String> {
        let mut task = match self.repo.find_of_user(id, user_id)? {
            Some(task) => task,
            None => return Err("no such task".to_string()),
        };

        task.complete();

        self.repo.save(&task)?;

        Ok(task)
    }
}

pub struct DeleteTask<'a> {
    repo: &'a mut Box<dyn TaskRepo>,
}

impl<'a> DeleteTask<'a> {
    pub fn new(repo: &'a mut Box<dyn TaskRepo>) -> Self {
        Self { repo }
    }

    pub fn invoke(&mut self, id: &str) -> Result<(), String> {
        self.repo.delete(id)
    }
}
