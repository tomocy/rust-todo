pub struct CreateUser<'a> {
    repo: &'a Box<dyn super::UserRepo>,
}

impl<'a> CreateUser<'a> {
    fn new(repo: &Box<dyn super::UserRepo>) -> CreateUser {
        CreateUser { repo: repo }
    }
}
