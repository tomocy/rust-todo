extern crate clap;

use super::super::super::usecase;
use super::super::super::{TaskRepo, UserRepo};
use super::super::controller;

pub struct App<'a> {
    user_repo: &'a mut Box<dyn UserRepo>,
    task_repo: &'a mut Box<dyn TaskRepo>,
    user_renderer: &'a Box<dyn controller::UserRenderer>,
    task_renderer: &'a Box<dyn controller::TaskRenderer>,
    session_manager: &'a mut Box<dyn controller::SessionManager>,
}

impl<'a> App<'a> {
    pub fn new(
        user_repo: &'a mut Box<dyn UserRepo>,
        task_repo: &'a mut Box<dyn TaskRepo>,
        user_renderer: &'a Box<dyn controller::UserRenderer>,
        task_renderer: &'a Box<dyn controller::TaskRenderer>,
        session_manager: &'a mut Box<dyn controller::SessionManager>,
    ) -> Self {
        Self {
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
            ("user", Some(args)) => self.run_user_command(args),
            ("task", Some(args)) => self.run_task_command(args),
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
            clap::SubCommand::with_name("login")
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
            clap::SubCommand::with_name("logout"),
            clap::SubCommand::with_name("delete"),
        ])
    }

    fn task_command<'b, 'c>(&self) -> clap::App<'b, 'c> {
        clap::SubCommand::with_name("task").subcommands(vec![
            clap::SubCommand::with_name("get"),
            clap::SubCommand::with_name("create").arg(
                clap::Arg::with_name("name")
                    .required(true)
                    .long("name")
                    .takes_value(true),
            ),
            clap::SubCommand::with_name("complete").arg(
                clap::Arg::with_name("id")
                    .required(true)
                    .long("id")
                    .takes_value(true),
            ),
            clap::SubCommand::with_name("delete").arg(
                clap::Arg::with_name("id")
                    .required(true)
                    .long("id")
                    .takes_value(true),
            ),
        ])
    }
}

impl<'a> App<'a> {
    fn run_user_command(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            ("create", Some(args)) => self.create_user(args),
            ("login", Some(args)) => self.authenticate_user(args),
            ("logout", Some(_)) => self.deauthenticate_user(),
            ("delete", Some(_)) => self.delete_user(),
            _ => Err("unknown command".to_string()),
        }
    }

    fn create_user(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let (email, password) = (
            args.value_of("email").unwrap(),
            args.value_of("password").unwrap(),
        );
        let user = usecase::CreateUser::new(self.user_repo)
            .invoke(email, password)
            .map_err(|err| format!("failed to create user: {}", err))?;

        self.session_manager
            .push_authenticated_user_id(&user.id())?;

        self.user_renderer
            .render_message("User is successfully created.");
        self.user_renderer.render_user(&user);

        Ok(())
    }

    fn authenticate_user(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let (email, password) = (
            args.value_of("email").unwrap(),
            args.value_of("password").unwrap(),
        );
        let user = usecase::AuthenticateUser::new(self.user_repo)
            .invoke(email, password)
            .map_err(|err| format!("failed to authenticate user: {}", err))?;

        match user {
            Some(user) => {
                self.session_manager
                    .push_authenticated_user_id(&user.id())?;

                self.user_renderer
                    .render_message("You are succefully logged in.");
                self.user_renderer.render_message("Take it easy!");
                self.user_renderer.render_user(&user);

                Ok(())
            }
            None => {
                self.user_renderer.render_error("Invalid credentials.");

                Ok(())
            }
        }
    }

    fn deauthenticate_user(&mut self) -> Result<(), String> {
        self.session_manager.drop_authenticated_user_id()?;
        self.user_renderer
            .render_message("You are successfully logged out.");
        self.user_renderer.render_message("See you later!");

        Ok(())
    }

    fn delete_user(&mut self) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.user_renderer
                    .render_error("authentication is required.");
                return Ok(());
            }
        };

        self.session_manager.drop_authenticated_user_id()?;
        usecase::DeleteUser::new(&mut self.user_repo, &mut self.task_repo).invoke(&user_id)?;

        self.user_renderer
            .render_message("Your data are completed deleted.");
        self.user_renderer.render_message("Take care of yourself.");

        Ok(())
    }
}

impl<'a> App<'a> {
    fn run_task_command(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            ("get", Some(_)) => self.get_tasks(),
            ("create", Some(args)) => self.create_task(args),
            ("complete", Some(args)) => self.complete_task(args),
            ("delete", Some(args)) => self.delete_task(args),
            _ => Err("unknown command".to_string()),
        }
    }

    fn get_tasks(&self) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.task_renderer
                    .render_error("authentication is required.");
                return Ok(());
            }
        };
        let tasks = usecase::GetTasks::new(self.task_repo)
            .invoke(&user_id)
            .map_err(|err| format!("failed to get tasks: {}", err))?;

        self.task_renderer.render_tasks(&tasks);

        Ok(())
    }

    fn create_task(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.task_renderer
                    .render_error("authentication is required.");
                return Ok(());
            }
        };
        let name = args.value_of("name").unwrap();
        let task = usecase::CreateTask::new(self.task_repo)
            .invoke(&user_id, name)
            .map_err(|err| format!("failed to create task: {}", err))?;

        self.task_renderer
            .render_message("Task is successfully created.");
        self.task_renderer.render_task(&task);

        Ok(())
    }

    fn complete_task(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let user_id = match self.session_manager.pop_authenticated_user_id()? {
            Some(user_id) => user_id,
            None => {
                self.task_renderer
                    .render_error("authentication is required.");
                return Ok(());
            }
        };
        let id = args.value_of("id").unwrap();
        let task = usecase::CompleteTask::new(&mut self.task_repo).invoke(id, &user_id)?;

        self.task_renderer.render_message("The task is completed.");
        self.task_renderer.render_task(&task);

        Ok(())
    }

    fn delete_task(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        if let None = self.session_manager.pop_authenticated_user_id()? {
            self.task_renderer
                .render_error("authentication is required.");
            return Ok(());
        }

        let id = args.value_of("id").unwrap();
        usecase::DeleteTask::new(&mut self.task_repo).invoke(id)?;

        self.task_renderer
            .render_message("The task is successfully deleted.");

        Ok(())
    }
}
