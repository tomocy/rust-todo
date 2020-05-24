extern crate serde;
extern crate serde_json;

use super::super::Hash;
use super::super::User as DomainUser;
use super::super::UserRepo as DomainUserRepo;
use super::rand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub struct UserRepo {
    workspace: String,
}

impl UserRepo {
    pub fn new(workspace: &str) -> Self {
        UserRepo {
            workspace: workspace.to_string(),
        }
    }

    fn load(&self) -> Result<Store, String> {
        let path = self.store_path()?;
        let mut store = String::new();

        self.init_store_file_if_not_exist()?;
        fs::File::open(path)
            .map_err(|err| err.to_string())?
            .read_to_string(&mut store)
            .map_err(|err| err.to_string())?;

        Ok(serde_json::from_str(&store).map_err(|err| err.to_string())?)
    }

    fn store(&self, store: &Store) -> Result<(), String> {
        let path = self.store_path()?;
        let store = serde_json::to_string(&store).map_err(|err| err.to_string())?;

        self.init_store_file_if_not_exist()?;
        fs::File::create(path)
            .map_err(|err| err.to_string())?
            .write_all(store.as_bytes())
            .map_err(|err| err.to_string())
    }

    fn init_store_file_if_not_exist(&self) -> Result<(), String> {
        let path = self.store_path()?;

        if Path::new(&path).exists() {
            return Ok(());
        }

        let store = serde_json::to_string(&Store {
            users: HashMap::new(),
        })
        .map_err(|err| err.to_string())?;

        fs::File::create(path)
            .map_err(|err| err.to_string())?
            .write_all(store.as_bytes())
            .map_err(|err| err.to_string())
    }

    fn store_path(&self) -> Result<String, String> {
        Ok(Path::new(&self.workspace)
            .join("store.json")
            .to_str()
            .ok_or("failed to locate store file")
            .unwrap()
            .to_string())
    }
}

impl DomainUserRepo for UserRepo {
    fn next_id(&self) -> Result<String, String> {
        Ok(rand::generate_string(50))
    }

    fn find_by_email(&self, email: &str) -> Result<Option<DomainUser>, String> {
        let store = self.load()?;
        for (_, user) in &store.users {
            if user.email == email {
                return Ok(Some(DomainUser::from(user.clone())));
            }
        }

        Ok(None)
    }

    fn save(&mut self, user: &DomainUser) -> Result<(), String> {
        let mut store = self.load()?;
        store
            .users
            .insert(user.id().clone(), User::from(user.clone()));

        self.store(&store)
    }
}

#[derive(Serialize, Deserialize)]
struct Store {
    users: HashMap<String, User>,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: String,
    email: String,
    password: String,
}

impl From<User> for DomainUser {
    fn from(user: User) -> Self {
        DomainUser::new(&user.id, &user.email, &Hash::from(user.password)).unwrap()
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
