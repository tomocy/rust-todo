extern crate todo;

use todo::infra::memory;
use todo::usecase;

#[test]
fn create_user() {
    let mut repo: Box<dyn todo::UserRepo> = Box::new(memory::UserRepo::new());

    let (email, password) = ("test@example.com", "aiueo");
    let user = usecase::CreateUser::new(&mut repo)
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
    let mut repo: Box<dyn todo::UserRepo> = Box::new(memory::UserRepo::new());

    let (email, password) = ("test@example.com", "aiueo");
    let created = usecase::CreateUser::new(&mut repo)
        .invoke(email, password)
        .unwrap();

    let user = usecase::AuthenticateUser::new(&repo)
        .invoke(email, password)
        .expect("should have succeeded to authenticate user")
        .expect("should have authenticated user");

    assert_eq!(created, user);
}

#[test]
fn delete_user() {
    let mut user_repo: Box<dyn todo::UserRepo> = Box::new(memory::UserRepo::new());
    let mut task_repo: Box<dyn todo::TaskRepo> = Box::new(memory::TaskRepo::new());

    let (email, password) = ("test@example.com", "aiueo");
    let user = usecase::CreateUser::new(&mut user_repo)
        .invoke(email, password)
        .unwrap();

    usecase::CreateTask::new(&mut task_repo)
        .invoke(user.id(), "test task name 1")
        .unwrap();
    usecase::CreateTask::new(&mut task_repo)
        .invoke(user.id(), "test task name 2")
        .unwrap();

    usecase::DeleteUser::new(&mut user_repo, &mut task_repo)
        .invoke(user.id())
        .expect("should have succeeded to delete user");

    assert_eq!(None, user_repo.find_by_email(user.email()).unwrap());
    assert_eq!(
        0,
        usecase::GetTasks::new(&task_repo)
            .invoke(user.id())
            .unwrap()
            .len()
    );
}

#[test]
fn get_tasks() {
    let mut repo: Box<dyn todo::TaskRepo> = Box::new(memory::TaskRepo::new());

    let user_id = "test user id";
    let created = usecase::CreateTask::new(&mut repo)
        .invoke(&user_id, "test task name")
        .unwrap();

    let tasks = usecase::GetTasks::new(&repo)
        .invoke(&user_id)
        .expect("should have succeeded to get tasks");

    assert_eq!(1, tasks.len());
    assert_eq!(created, *tasks.get(0).unwrap());
}

#[test]
fn create_task() {
    let mut repo: Box<dyn todo::TaskRepo> = Box::new(memory::TaskRepo::new());

    let (user_id, name) = ("test user id", "test task name");
    let task = usecase::CreateTask::new(&mut repo)
        .invoke(user_id, name)
        .expect("should have succeeded to create task");

    assert_eq!(user_id, task.user_id());
    assert_eq!(name, task.name());
}

#[test]
fn complete_task() {
    let mut repo: Box<dyn todo::TaskRepo> = Box::new(memory::TaskRepo::new());

    let user_id = "test user id";
    let created = usecase::CreateTask::new(&mut repo)
        .invoke(&user_id, "test task name")
        .expect("should have succeeded to create task");

    let task = usecase::CompleteTask::new(&mut repo)
        .invoke(created.id(), &user_id)
        .expect("should have succeeded to complete task");

    assert!(task.is_completed());
}

#[test]
fn delete_task() {
    let mut repo: Box<dyn todo::TaskRepo> = Box::new(memory::TaskRepo::new());
    let user_id = "test user id";
    let created = usecase::CreateTask::new(&mut repo)
        .invoke(&user_id, "test task name")
        .unwrap();

    usecase::DeleteTask::new(&mut repo)
        .invoke(&created.id(), &user_id)
        .expect("should have succeeded to delete task");

    let got = usecase::GetTasks::new(&repo).invoke(&user_id).unwrap();
    assert_eq!(0, got.len());
}
