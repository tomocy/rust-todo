pub struct User {
    id: String,
}

impl User {
    pub fn new(id: &str) -> Result<User, String> {
        Self::verify_id(id)?;

        Ok(User {
            id: String::from(id),
        })
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    fn verify_id(id: &str) -> Result<(), String> {
        match id {
            "" => Err(String::from("id should not be empty")),
            _ => Ok(()),
        }
    }
}
