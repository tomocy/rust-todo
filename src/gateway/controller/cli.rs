extern crate clap;

use super::super::super::usecase;
use super::super::super::{TaskRepo, UserRepo};

pub struct App<'a> {
    user_repo: &'a mut Box<dyn UserRepo>,
    task_repo: &'a mut Box<dyn TaskRepo>,
    user_renderer: &'a Box<dyn super::UserRenderer>,
    session_manager: &'a mut Box<dyn super::SessionManager>,
}

impl<'a> App<'a> {
    pub fn new(
        user_repo: &'a mut Box<dyn UserRepo>,
        task_repo: &'a mut Box<dyn TaskRepo>,
        user_renderer: &'a Box<dyn super::UserRenderer>,
        session_manager: &'a mut Box<dyn super::SessionManager>,
    ) -> Self {
        App {
            user_repo,
            task_repo,
            user_renderer,
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
                let mut app = TaskApp::new(self.task_repo, self.session_manager);
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
        clap::SubCommand::with_name("task").subcommands(vec![clap::SubCommand::with_name("create")
            .arg(
                clap::Arg::with_name("name")
                    .required(true)
                    .long("name")
                    .takes_value(true),
            )])
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
    session_manager: &'a Box<dyn super::SessionManager>,
}

impl<'a> TaskApp<'a> {
    fn new(
        repo: &'a mut Box<dyn TaskRepo>,
        session_manager: &'a mut Box<dyn super::SessionManager>,
    ) -> Self {
        TaskApp {
            repo,
            session_manager,
        }
    }

    fn run(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            ("create", Some(args)) => self.create(args),
            _ => Err("unknown command".to_string()),
        }
    }

    fn create(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
        let user_id = self.pop_authenticated_user_id()?;
        let name = args.value_of("name").unwrap();
        let _task = usecase::CreateTask::new(self.repo)
            .invoke(&user_id, name)
            .map_err(|err| format!("failed to create task: {}", err))?;

        Ok(())
    }

    fn pop_authenticated_user_id(&self) -> Result<String, String> {
        if let Some(user_id) = self.session_manager.pop_authenticated_user_id()? {
            Ok(user_id)
        } else {
            Err("authentication is required.".to_string())
        }
    }
}
