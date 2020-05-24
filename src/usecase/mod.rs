pub struct CreateUser<'a> {
    repo: &'a mut Box<dyn super::UserRepo>,
}

impl<'a> CreateUser<'a> {
    pub fn new(repo: &'a mut Box<dyn super::UserRepo>) -> CreateUser<'a> {
        CreateUser { repo: repo }
    }

    pub fn invoke(&mut self, email: &str, password: &str) -> Result<super::User, String> {
        let id = self.repo.next_id()?;
        let password = super::Hash::new(password)?;
        let user = super::User::new(&id, email, &password)?;

        self.repo.save(&user)?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::super::infra::memory;
    use super::super::*;
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
}
