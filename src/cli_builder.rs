use crate::config::{CliConfig, CommandType};
use clap::{Arg, Command};

pub fn build_cli(cli_config: &CliConfig) -> Command {
    // Pass a string slice directly to the Command constructor
    let mut app = Command::new(cli_config.app_name)
        .version(cli_config.version)
        .about(cli_config.about);

    for cmd in cli_config.commands.iter() {
        let mut command = Command::new(cmd.name).about(cmd.description);

        for arg in cmd.args.iter() {
            let argument = Arg::new(&*arg.name).help(&arg.help).required(arg.required);
            command = command.arg(argument);
        }

        app = app.subcommand(command);
    }

    app
}

pub fn execute_cli(cli_config: &CliConfig) {
    let matches = build_cli(cli_config).get_matches();
    if let Some((command_name, sub_matches)) = matches.subcommand() {
        if let Some(cmd) = cli_config
            .commands()
            .iter()
            .find(|cmd| cmd.name == command_name)
        {
            match cmd.command_type {
                CommandType::Standard(_) => {
                    let args = sub_matches
                        .get_many::<String>("message")
                        .map(|vals| vals.cloned().collect::<Vec<_>>())
                        .unwrap_or_default();
                    cmd.execute_action(Some(&args));
                }
                CommandType::UI(_) => {
                    cmd.execute_action(None);
                }
                CommandType::UIWithReturn(_) => {
                    cmd.execute_action(None);
                }
            }
        } else {
            eprintln!("Command '{}' not recognized.", command_name);
        }
    } else {
        eprintln!("No subcommand was used.");
    }
}
