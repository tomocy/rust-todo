pub mod controller {
    use super::super::User;

    pub trait UserRenderer {
        fn render_user(&self, user: &User);
    }

    pub mod cli {
        extern crate clap;

        use super::super::super::usecase;
        use super::super::super::UserRepo;

        pub struct App<'a> {
            user_repo: &'a mut Box<dyn UserRepo>,
        }

        impl<'a> App<'a> {
            pub fn new(user_repo: &'a mut Box<dyn UserRepo>) -> App<'a> {
                App { user_repo }
            }
            pub fn run(&mut self) -> Result<(), String> {
                let args = self.app().get_matches();
                match args.subcommand() {
                    ("user", Some(args)) => self.run_user_command(args),
                    _ => Err("unknown command".to_string()),
                }
            }

            fn run_user_command(&mut self, args: &clap::ArgMatches) -> Result<(), String> {
                match args.subcommand() {
                    ("create", Some(args)) => {
                        let (email, password) = (
                            args.value_of("email").unwrap(),
                            args.value_of("password").unwrap(),
                        );
                        let user = usecase::CreateUser::new(self.user_repo)
                            .invoke(email, password)
                            .map_err(|err| format!("failed to create user: {}", err));

                        Ok(())
                    }
                    _ => Err("unknown command".to_string()),
                }
            }

            fn app<'b, 'c>(&self) -> clap::App<'b, 'c> {
                clap::App::new("todo").subcommands(vec![self.user_command()])
            }

            fn user_command<'b, 'c>(&self) -> clap::App<'b, 'c> {
                clap::SubCommand::with_name("user").subcommands(vec![clap::SubCommand::with_name(
                    "create",
                )
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
                )])
            }
        }
    }
}
