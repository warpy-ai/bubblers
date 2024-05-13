use crate::config::CliConfig;
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
