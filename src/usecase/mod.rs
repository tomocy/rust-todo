pub struct CreateUser<'a> {
    repo: &'a mut Box<dyn super::UserRepo>,
}

impl<'a> CreateUser<'a> {
    pub fn new(repo: &'a mut Box<dyn super::UserRepo>) -> Self {
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

pub struct AuthenticateUser<'a> {
    repo: &'a Box<dyn super::UserRepo>,
}

impl<'a> AuthenticateUser<'a> {
    pub fn new(repo: &'a Box<dyn super::UserRepo>) -> Self {
        AuthenticateUser { repo }
    }

    pub fn invoke(&self, email: &str, password: &str) -> Result<Option<super::User>, String> {
        let user = self.repo.find_by_email(email)?;
        match user {
            Some(user) if user.password().verify(password)? => Ok(Some(user)),
            _ => Ok(None),
        }
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

    #[test]
    fn authenticate_user() {
        let mut repo: Box<dyn UserRepo> = Box::new(memory::UserRepo::new());

        let (email, password) = ("test@example.com", "aiueo");
        let created = CreateUser::new(&mut repo).invoke(email, password).unwrap();

        let user = AuthenticateUser::new(&repo)
            .invoke(email, password)
            .expect("should have succeeded to authenticate user")
            .expect("should have authenticated user");

        assert_eq!(created.id(), user.id());
        assert_eq!(created.email(), user.email());
    }
}
