pub struct User {
    id: String,
    email: String,
}

impl User {
    pub fn new(id: &str, email: &str) -> Result<User, String> {
        Self::verify_id(id)?;
        Self::verify_email(email)?;

        Ok(User {
            id: String::from(id),
            email: String::from(email),
        })
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    fn verify_id(id: &str) -> Result<(), String> {
        match id {
            "" => Err(String::from("id should not be empty")),
            _ => Ok(()),
        }
    }

    fn verify_email(email: &str) -> Result<(), String> {
        match email {
            "" => Err(String::from("email should not be empty")),
            _ => Ok(()),
        }
    }
}
