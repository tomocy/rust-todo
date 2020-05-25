extern crate clap;

use super::super::super::usecase;
use super::super::super::{TaskRepo, UserRepo};

pub struct App<'a> {
    user_repo: &'a mut Box<dyn UserRepo>,
    task_repo: &'a mut Box<dyn TaskRepo>,
    user_renderer: &'a Box<dyn super::UserRenderer>,
    task_renderer: &'a Box<dyn super::TaskRenderer>,
    session_manager: &'a mut Box<dyn super::SessionManager>,
}

impl<'a> App<'a> {
    pub fn new(
        user_repo: &'a mut Box<dyn UserRepo>,
        task_repo: &'a mut Box<dyn TaskRepo>,
        user_renderer: &'a Box<dyn super::UserRenderer>,
        task_renderer: &'a Box<dyn super::TaskRenderer>,
        session_manager: &'a mut Box<dyn super::SessionManager>,
    ) -> Self {
        App {
            user_repo,
            task_repo,
            user_renderer,
            task_renderer,
            session_manager,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let args = self.app().get_matches();
        match args.subcommand() {
            ("user", Some(args)) => {
                let mut app =
                    UserApp::new(self.user_repo, self.user_renderer, self.session_manager);
                app.run(args)
            }
            ("task", Some(args)) => {
                let mut app =
                    TaskApp::new(self.task_repo, self.task_renderer, self.session_manager);
                app.run(args)
            }
            _ => Err("unknown command".to_string()),
        }
    }

    fn app<'b, 'c>(&self) -> clap::App<'b, 'c> {
        clap::App::new("todo").subcommands(vec![self.user_command(), self.task_command()])
    }

    fn user_command<'b, 'c>(&self) -> clap::App<'b, 'c> {
        clap::SubCommand::with_name("user").subcommands(vec![
            clap::SubCommand::with_name("create")
                .arg(
                    clap::Arg::with_name("email")
                        .required(true)
                        .long("email")
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("password")
                        .required(true)
                        .long("password")
                        .takes_value(true),
                ),
            clap::SubCommand::with_name("authenticate")
                .arg(
                    clap::Arg::with_name("email")
                        .required(true)
                        .long("email")
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("password")
                        .required(true)
                        .long("password")
                        .takes_value(true),
                ),
        ])
    }

    fn task_command<'b, 'c>(&self) -> clap::App<'b, 'c> {
        clap::SubCommand::with_name("task").subcommands(vec![
            clap::SubCommand::with_name("create").arg(
                clap::Arg::with_name("name")
                    .required(true)
                    .long("name")
                    .takes_value(true),
            ),
            clap::SubCommand::with_name("get"),
            clap::SubCommand::with_name("delete").arg(
                clap::Arg::with_name("id")
                    .required(true)
                    .long("id")
                    .takes_value(true),
            ),
        ])
    }
}

struct UserApp<'a> {
    repo: &'a mut Box<dyn UserRepo>,
    renderer: &'a Box<dyn super::UserRenderer>,
    session_manager: &'a mut Box<dyn super::SessionManager>,
}

impl<'a> UserApp<'a> {
    fn new(
        repo: &'a mut Box<dyn UserRepo>,
        renderer: &'a Box<dyn super::UserRenderer>,
        session_manager: &'a mut Box<dyn super::SessionManager>,
    ) -> Self {
        UserApp {
            repo,
            renderer,
            session_manager,
        }
    }

    fn run(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            ("create", Some(args)) => self.create(args),
            ("authenticate", Some(args)) => self.authenticate(args),
            _ => Err("unknown command".to_string()),
        }
    }

    fn create(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let (email, password) = (
            args.value_of("email").unwrap(),
            args.value_of("password").unwrap(),
        );
        let user = usecase::CreateUser::new(self.repo)
            .invoke(email, password)
            .map_err(|err| format!("failed to create user: {}", err))?;

        self.session_manager
            .push_authenticated_user_id(&user.id())?;

        self.renderer
            .render_message("User is successfully created.");
        self.renderer.render_user(&user);

        Ok(())
    }

    fn authenticate(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let (email, password) = (
            args.value_of("email").unwrap(),
            args.value_of("password").unwrap(),
        );
        let user = usecase::AuthenticateUser::new(self.repo)
            .invoke(email, password)
            .map_err(|err| format!("failed to authenticate user: {}", err))?;

        match user {
            Some(user) => {
                self.session_manager
                    .push_authenticated_user_id(&user.id())?;

                self.renderer
                    .render_message("User is successfully created.");
                self.renderer.render_user(&user);

                Ok(())
            }
            None => {
                self.renderer.render_error("Invalid credentials.");

                Ok(())
            }
        }
    }
}

struct TaskApp<'a> {
    repo: &'a mut Box<dyn TaskRepo>,
    renderer: &'a Box<dyn super::TaskRenderer>,
    session_manager: &'a Box<dyn super::SessionManager>,
}

impl<'a> TaskApp<'a> {
    fn new(
        repo: &'a mut Box<dyn TaskRepo>,
        renderer: &'a Box<dyn super::TaskRenderer>,
        session_manager: &'a mut Box<dyn super::SessionManager>,
    ) -> Self {
        TaskApp {
            repo,
            renderer,
            session_manager,
        }
    }

    fn run(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            ("create", Some(args)) => self.create(args),
            ("get", Some(_)) => self.get(),
            ("delete", Some(args)) => self.delete(args),
            _ => Err("unknown command".to_string()),
        }
    }

    fn get(&self) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.renderer.render_error("authentication is required.");
                return Ok(());
            }
        };
        let tasks = usecase::GetTasks::new(self.repo)
            .invoke(&user_id)
            .map_err(|err| format!("failed to get tasks: {}", err))?;

        self.renderer.render_tasks(&tasks);

        Ok(())
    }

    fn create(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.renderer.render_error("authentication is required.");
                return Ok(());
            }
        };
        let name = args.value_of("name").unwrap();
        let task = usecase::CreateTask::new(self.repo)
            .invoke(&user_id, name)
            .map_err(|err| format!("failed to create task: {}", err))?;

        self.renderer
            .render_message("Task is successfully created.");
        self.renderer.render_task(&task);

        Ok(())
    }

    fn delete(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        if let None = self.session_manager.pop_authenticated_user_id()? {
            self.renderer.render_error("authentication is required.");
            return Ok(());
        }

        let id = args.value_of("id").unwrap();
        usecase::DeleteTask::new(&mut self.repo).invoke(id)?;

        self.renderer
            .render_message("The task is successfully deleted.");

        Ok(())
    }
}
