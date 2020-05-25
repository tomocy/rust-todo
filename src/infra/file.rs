extern crate serde;
extern crate serde_json;

use super::super::gateway::controller;
use super::super::Hash;
use super::super::Task as DomainTask;
use super::super::TaskRepo as DomainTaskRepo;
use super::super::User as DomainUser;
use super::super::UserRepo as DomainUserRepo;
use super::rand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub struct UserRepo {
    file: File,
}

impl UserRepo {
    pub fn new(workspace: &str) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            file: File::new(workspace)?,
        })
    }
}

impl DomainUserRepo for UserRepo {
    fn next_id(&self) -> Result<String, Box<dyn error::Error>> {
        Ok(rand::generate_string(50))
    }

    fn find_by_email(&self, email: &str) -> Result<Option<DomainUser>, Box<dyn error::Error>> {
        let store = self.file.load()?;
        for (_, user) in &store.users {
            if user.email == email {
                return Ok(Some(DomainUser::from(user.clone())));
            }
        }

        Ok(None)
    }

    fn save(&mut self, user: &DomainUser) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store
            .users
            .insert(user.id().clone(), User::from(user.clone()));

        self.file.store(&store)
    }

    fn delete(&mut self, id: &str) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store.users.remove(id);

        self.file.store(&store)
    }
}

pub struct TaskRepo {
    file: File,
}

impl TaskRepo {
    pub fn new(workspace: &str) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            file: File::new(workspace)?,
        })
    }
}

impl DomainTaskRepo for TaskRepo {
    fn next_id(&self) -> Result<String, Box<dyn error::Error>> {
        Ok(rand::generate_string(70))
    }

    fn get(&self, user_id: &str) -> Result<Vec<DomainTask>, Box<dyn error::Error>> {
        let store = self.file.load()?;

        let mut tasks = Vec::new();
        for (_, task) in store.tasks {
            if task.user_id != user_id {
                continue;
            }

            tasks.push(DomainTask::from(task));
        }

        Ok(tasks)
    }

    fn find_of_user(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<Option<DomainTask>, Box<dyn error::Error>> {
        let store = self.file.load()?;
        for (_, task) in store.tasks {
            if task.id != id || task.user_id != user_id {
                continue;
            }

            return Ok(Some(DomainTask::from(task)));
        }

        Ok(None)
    }

    fn save(&mut self, task: &DomainTask) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store
            .tasks
            .insert(task.id().clone(), Task::from(task.clone()));

        self.file.store(&store)
    }

    fn delete(&mut self, id: &str) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store.tasks.remove(id);

        self.file.store(&store)
    }

    fn delete_of_user(&mut self, user_id: &str) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        let ids: Vec<String> = store
            .tasks
            .values()
            .filter(|task| task.user_id == user_id)
            .map(|task| task.id.clone())
            .collect();
        for id in ids {
            store.tasks.remove(&id);
        }

        self.file.store(&store)
    }
}

pub struct SessionManager {
    file: File,
}

impl SessionManager {
    pub fn new(workspace: &str) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            file: File::new(workspace)?,
        })
    }
}

impl controller::SessionManager for SessionManager {
    fn push_authenticated_user_id(&mut self, user_id: &str) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store.session.authenticated_user_id = user_id.to_string();

        self.file.store(&store)
    }

    fn pop_authenticated_user_id(&self) -> Result<Option<String>, Box<dyn error::Error>> {
        let store = self.file.load()?;
        let user_id = store.session.authenticated_user_id;
        if user_id.is_empty() {
            Ok(None)
        } else {
            Ok(Some(user_id))
        }
    }

    fn drop_authenticated_user_id(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mut store = self.file.load()?;
        store.session.authenticated_user_id = "".to_string();

        self.file.store(&store)
    }
}

struct File {
    workspace: String,
}

impl File {
    fn new(workspace: &str) -> Result<Self, Box<dyn error::Error>> {
        let file = Self {
            workspace: workspace.to_string(),
        };

        file.init_store_file_if_not_exist()?;

        Ok(file)
    }

    fn init_store_file_if_not_exist(&self) -> Result<(), Box<dyn error::Error>> {
        let path = self.store_path()?;

        if Path::new(&path).exists() {
            return Ok(());
        }

        let store = serde_json::to_string(&Store::new())?;
        fs::File::create(path)?.write_all(store.as_bytes())?;

        Ok(())
    }

    fn load(&self) -> Result<Store, Box<dyn error::Error>> {
        let path = self.store_path()?;
        let mut store = String::new();

        self.init_store_file_if_not_exist()?;
        fs::File::open(path)
            .map_err(|err| err.to_string())?
            .read_to_string(&mut store)
            .map_err(|err| err.to_string())?;

        Ok(serde_json::from_str(&store).map_err(|err| err.to_string())?)
    }

    fn store(&self, store: &Store) -> Result<(), Box<dyn error::Error>> {
        let path = self.store_path()?;
        let store = serde_json::to_string(store).map_err(|err| err.to_string())?;

        self.init_store_file_if_not_exist()?;
        fs::File::create(path)?.write_all(store.as_bytes())?;

        Ok(())
    }

    fn store_path(&self) -> Result<String, Box<dyn error::Error>> {
        Ok(Path::new(&self.workspace)
            .join("store.json")
            .to_str()
            .ok_or("failed to locate store file")
            .unwrap()
            .to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct Store {
    users: HashMap<String, User>,
    tasks: HashMap<String, Task>,
    session: Session,
}

impl Store {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            tasks: HashMap::new(),
            session: Session::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: String,
    email: String,
    password: String,
}

impl From<User> for DomainUser {
    fn from(user: User) -> Self {
        DomainUser {
            id: user.id,
            email: user.email,
            password: Hash::from(user.password),
        }
    }
}

impl From<DomainUser> for User {
    fn from(user: DomainUser) -> Self {
        User {
            id: user.id().clone(),
            email: user.email().clone(),
            password: user.password().clone().0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: String,
    user_id: String,
    name: String,
    completed: bool,
}

impl From<Task> for DomainTask {
    fn from(task: Task) -> Self {
        DomainTask {
            id: task.id,
            user_id: task.user_id,
            name: task.name,
            completed: task.completed,
        }
    }
}

impl From<DomainTask> for Task {
    fn from(task: DomainTask) -> Self {
        Task {
            id: task.id().clone(),
            user_id: task.user_id().clone(),
            name: task.name().clone(),
            completed: task.is_completed(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Session {
    authenticated_user_id: String,
}

impl Session {
    fn new() -> Self {
        Self {
            authenticated_user_id: String::new(),
        }
    }
}
