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

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::infra::memory;
    use super::*;
    #[test]
    fn create_user() {
        let mut repo: Box<dyn UserRepo> = Box::new(memory::UserRepo::new());

        let (email, password) = ("test@example.com", "aiueo");
        let user = CreateUser::new(&mut repo)
            .invoke(email, password)
            .expect("should have created user");

        assert_eq!(email, user.email());
        assert!(user
            .password()
            .verify(password)
            .expect("should have verified password"));
    }

    #[test]
    fn authenticate_user() {
        let mut repo: Box<dyn UserRepo> = Box::new(memory::UserRepo::new());

        let (email, password) = ("test@example.com", "aiueo");
        let created = CreateUser::new(&mut repo).invoke(email, password).unwrap();

        let user = AuthenticateUser::new(&repo)
            .invoke(email, password)
            .expect("should have succeeded to authenticate user")
            .expect("should have authenticated user");

        assert_eq!(created, user);
    }

    #[test]
    fn delete_user() {
        let mut user_repo: Box<dyn UserRepo> = Box::new(memory::UserRepo::new());
        let mut task_repo: Box<dyn TaskRepo> = Box::new(memory::TaskRepo::new());

        let (email, password) = ("test@example.com", "aiueo");
        let user = CreateUser::new(&mut user_repo)
            .invoke(email, password)
            .unwrap();

        CreateTask::new(&mut task_repo)
            .invoke(user.id(), "test task name 1")
            .unwrap();
        CreateTask::new(&mut task_repo)
            .invoke(user.id(), "test task name 2")
            .unwrap();

        DeleteUser::new(&mut user_repo, &mut task_repo)
            .invoke(user.id())
            .expect("should have succeeded to delete user");

        assert_eq!(None, user_repo.find_by_email(user.email()).unwrap());
        assert_eq!(
            0,
            GetTasks::new(&task_repo).invoke(user.id()).unwrap().len()
        );
    }

    #[test]
    fn get_tasks() {
        let mut repo: Box<dyn TaskRepo> = Box::new(memory::TaskRepo::new());

        let user_id = "test user id";
        let created = CreateTask::new(&mut repo)
            .invoke(&user_id, "test task name")
            .unwrap();

        let tasks = GetTasks::new(&repo)
            .invoke(&user_id)
            .expect("should have succeeded to get tasks");

        assert_eq!(1, tasks.len());
        assert_eq!(created, *tasks.get(0).unwrap());
    }

    #[test]
    fn create_task() {
        let mut repo: Box<dyn TaskRepo> = Box::new(memory::TaskRepo::new());

        let (user_id, name) = ("test user id", "test task name");
        let task = CreateTask::new(&mut repo)
            .invoke(user_id, name)
            .expect("should have succeeded to create task");

        assert_eq!(user_id, task.user_id());
        assert_eq!(name, task.name());
    }

    #[test]
    fn complete_task() {
        let mut repo: Box<dyn TaskRepo> = Box::new(memory::TaskRepo::new());

        let user_id = "test user id";
        let created = CreateTask::new(&mut repo)
            .invoke(&user_id, "test task name")
            .expect("should have succeeded to create task");

        let task = CompleteTask::new(&mut repo)
            .invoke(created.id(), &user_id)
            .expect("should have succeeded to complete task");

        assert!(task.is_completed());
    }

    #[test]
    fn delete_task() {
        let mut repo: Box<dyn TaskRepo> = Box::new(memory::TaskRepo::new());
        let user_id = "test user id";
        let created = CreateTask::new(&mut repo)
            .invoke(&user_id, "test task name")
            .unwrap();

        DeleteTask::new(&mut repo)
            .invoke(&created.id())
            .expect("should have succeeded to delete task");

        let got = GetTasks::new(&repo).invoke(&user_id).unwrap();
        assert_eq!(0, got.len());
    }
}
