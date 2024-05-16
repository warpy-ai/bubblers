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
            let args: Vec<String> = cmd
                .args
                .iter()
                .filter_map(|arg| {
                    sub_matches
                        .get_one::<String>(arg.name)
                        .map(|s| s.to_string())
                })
                .collect();

            match &cmd.command_type {
                CommandType::Standard(_) => {
                    if args.is_empty() {
                        cmd.execute_action(None);
                    } else {
                        cmd.execute_action(Some(&args));
                    }
                }
                CommandType::UI(_) => {
                    if args.is_empty() {
                        cmd.execute_action(None);
                    } else {
                        cmd.execute_action(Some(&args));
                    }
                }
                CommandType::UIWithReturn(_) => {
                    if args.is_empty() {
                        cmd.execute_action(None);
                    } else {
                        cmd.execute_action(Some(&args));
                    }
                }
            }
        } else {
            eprintln!("Command '{}' not recognized.", command_name);
        }
    } else {
        eprintln!("No subcommand was used.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ArgConfig, CliConfig, CommandConfig, CommandType};
    use std::sync::Arc;

    fn get_test_cli_config() -> CliConfig<'static> {
        CliConfig {
            app_name: "test_app",
            version: "1.0",
            about: "A test application",
            commands: vec![CommandConfig {
                name: "test_cmd",
                description: "A test command",
                args: vec![ArgConfig {
                    name: "message",
                    help: "A test message".to_string(),
                    required: true,
                }],
                command_type: CommandType::Standard(Arc::new(|args| {
                    println!("Executing test_cmd with args: {:?}", args);
                })),
            }],
        }
    }

    #[test]
    fn test_build_cli() {
        let cli_config = get_test_cli_config();
        let app = build_cli(&cli_config);

        // Check the main command attributes
        assert_eq!(app.get_name(), "test_app");
        assert_eq!(
            app.get_about().map(|s| s.to_string()),
            Some("A test application".to_string())
        );
        assert_eq!(
            app.get_version().map(|s| s.to_string()),
            Some("1.0".to_string())
        );

        // Check the subcommand attributes
        let subcommand = app.get_subcommands().find(|sc| sc.get_name() == "test_cmd");
        assert!(subcommand.is_some());
        let subcommand = subcommand.unwrap();
        assert_eq!(
            subcommand.get_about().map(|s| s.to_string()),
            Some("A test command".to_string())
        );

        // Check the argument attributes
        let argument = subcommand
            .get_arguments()
            .find(|arg| arg.get_id() == "message");
        assert!(argument.is_some());
        let argument = argument.unwrap();
        assert_eq!(
            argument.get_help().map(|s| s.to_string()),
            Some("A test message".to_string())
        );
        assert!(argument.is_required_set());
    }

    #[test]
    fn test_execute_cli_unrecognized_command() {
        let cli_config = get_test_cli_config();
        let app = build_cli(&cli_config);

        // Capture the output to check the error message
        let result = app.try_get_matches_from(vec!["test_app", "unknown_cmd"]);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("unrecognized subcommand"));
        }
    }

    #[test]
    fn test_execute_cli_arguments_not_provided() {
        let cli_config = get_test_cli_config();
        let app = build_cli(&cli_config);

        // Capture the output to check the error message
        let result = app.try_get_matches_from(vec!["test_app", "test_cmd"]);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err
                .to_string()
                .contains("the following required arguments were not provided"));
        }
    }

    #[test]
    fn test_execute_cli_no_subcommand() {
        let cli_config = get_test_cli_config();
        let cmd = build_cli(&cli_config);

        let result = cmd.try_get_matches_from(vec!["test_app"]);

        assert!(result.is_ok());
        if let Ok(matches) = result {
            assert!(matches.subcommand_matches("test_cmd").is_none());
        }
    }
}
