use bcrypt;

pub mod gateway;
pub mod infra;
pub mod usecase;

pub trait UserRepo {
    fn next_id(&self) -> Result<String, String>;
    fn save(&mut self, user: &User) -> Result<(), String>;
}

#[derive(Clone)]
pub struct User {
    id: String,
    email: String,
    password: Hash,
}

impl User {
    pub fn new(id: &str, email: &str, password: &Hash) -> Result<User, String> {
        Self::verify_id(id)?;
        Self::verify_email(email)?;

        Ok(User {
            id: String::from(id),
            email: String::from(email),
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

#[derive(Clone)]
pub struct Hash(String);

impl Hash {
    pub fn new(plain: &str) -> Result<Hash, String> {
        verify_not_empty(plain)?;

        let hashed = bcrypt::hash(plain, bcrypt::DEFAULT_COST).map_err(|err| err.to_string())?;
        Ok(Hash(hashed))
    }

    pub fn verify(&self, plain: &str) -> Result<bool, String> {
        let valid = bcrypt::verify(plain, &self.0).map_err(|err| err.to_string())?;
        Ok(valid)
    }
}

fn verify_not_empty(s: &str) -> Result<(), String> {
    match s {
        "" => Err(String::from("empty")),
        _ => Ok(()),
    }
}
