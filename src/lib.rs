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
        verify_not_empty(id).map_err(|_| "id should not be empty")?;

        Ok(())
    }

    fn verify_email(email: &str) -> Result<(), String> {
        verify_not_empty(email).map_err(|_| "email should not be empty")?;

        Ok(())
    }
}

fn verify_not_empty(s: &str) -> Result<(), String> {
    match s {
        "" => Err(String::from("empty")),
        _ => Ok(()),
    }
}
