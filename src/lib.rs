pub mod gateway;
pub mod infra;
pub mod usecase;

use bcrypt;

pub trait UserRepo {
    fn next_id(&self) -> Result<String, String>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    fn save(&mut self, user: &User) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: String,
    email: String,
    password: Hash,
}

impl User {
    pub fn new(id: &str, email: &str, password: &Hash) -> Result<Self, String> {
        Self::verify_id(id)?;
        Self::verify_email(email)?;

        Ok(Self {
            id: id.to_string(),
            email: email.to_string(),
            password: password.clone(),
        })
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &Hash {
        &self.password
    }

    fn verify_id(id: &str) -> Result<(), String> {
        verify_not_empty(id).map_err(|_| "id should not be empty")?;

        Ok(())
    }

    fn verify_email(email: &str) -> Result<(), String> {
        verify_not_empty(email).map_err(|_| "email should not be empty")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hash(String);

impl Hash {
    pub fn new(plain: &str) -> Result<Self, String> {
        verify_not_empty(plain)?;

        let hashed = bcrypt::hash(plain, bcrypt::DEFAULT_COST).map_err(|err| err.to_string())?;
        Ok(Self(hashed))
    }

    pub fn verify(&self, plain: &str) -> Result<bool, String> {
        let valid = bcrypt::verify(plain, &self.0).map_err(|err| err.to_string())?;
        Ok(valid)
    }
}

impl From<String> for Hash {
    fn from(hash: String) -> Self {
        Self(hash)
    }
}

pub trait TaskRepo {
    fn next_id(&self) -> Result<String, String>;
    fn get(&self, user_id: &str) -> Result<Vec<Task>, String>;
    fn find_of_user(&self, id: &str, user_id: &str) -> Result<Option<Task>, String>;
    fn save(&mut self, task: &Task) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
    fn delete_of_user(&mut self, user_id: &str) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: String,
    user_id: String,
    name: String,
    completed: bool,
}

impl Task {
    pub fn new(id: &str, user_id: &str, name: &str) -> Result<Self, String> {
        Self::verify_id(id)?;
        Self::verify_user_id(user_id)?;
        Self::verify_name(name)?;

        Ok(Self {
            id: id.to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            completed: false,
        })
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    fn verify_id(id: &str) -> Result<(), String> {
        verify_not_empty(id).map_err(|_| "id should not be empty")?;

        Ok(())
    }

    fn verify_user_id(user_id: &str) -> Result<(), String> {
        verify_not_empty(user_id).map_err(|_| "user id should not be empty")?;

        Ok(())
    }

    fn verify_name(name: &str) -> Result<(), String> {
        verify_not_empty(name).map_err(|_| "name id should not be empty")?;

        Ok(())
    }
}

fn verify_not_empty(s: &str) -> Result<(), String> {
    match s {
        "" => Err(String::from("empty")),
        _ => Ok(()),
    }
}
