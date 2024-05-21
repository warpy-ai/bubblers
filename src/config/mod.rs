use std::{io, sync::Arc};

use crate::wrappers::{show_input_form, text_area};

#[derive(Clone)]
pub struct ArgConfig {
    pub name: &'static str,
    pub help: String,
    pub required: bool,
}

#[derive(Clone)]
pub enum CommandType<'a> {
    Standard(Arc<dyn Fn(&[String]) + Send + Sync + 'a>),
    UI(Arc<dyn Fn() -> Result<(), io::Error> + Send + Sync + 'a>),
    UIWithReturn(Arc<dyn Fn() -> Result<String, io::Error> + Send + Sync + 'a>),
}

#[derive(Clone)]
pub struct CommandConfig<'a> {
    pub name: &'static str,
    pub description: &'static str,
    pub args: Vec<ArgConfig>,
    pub command_type: CommandType<'a>,
}

impl<'a> CommandConfig<'a> {
    pub fn new_standard(
        name: &'static str,
        description: &'static str,
        action: Arc<dyn Fn(&[String]) + Send + Sync + 'a>,
    ) -> Self {
        CommandConfig {
            name,
            description,
            args: Vec::new(),
            command_type: CommandType::Standard(action),
        }
    }

    pub fn new_ui(
        name: &'static str,
        description: &'static str,
        action: Arc<dyn Fn() -> Result<(), io::Error> + Send + Sync + 'a>,
    ) -> Self {
        CommandConfig {
            name,
            description,
            args: Vec::new(),
            command_type: CommandType::UI(action),
        }
    }

    pub fn new_ui_with_return(
        name: &'static str,
        description: &'static str,
        action: Arc<dyn Fn() -> Result<String, io::Error> + Send + Sync + 'a>,
    ) -> Self {
        CommandConfig {
            name,
            description,
            args: Vec::new(),
            command_type: CommandType::UIWithReturn(action),
        }
    }

    pub fn add_arg(&mut self, arg: ArgConfig) -> &mut Self {
        self.args.push(arg);
        self
    }

    // Executes the action associated with this command
    pub fn execute_action(&self, args: Option<&[String]>) {
        match &self.command_type {
            CommandType::Standard(action) => {
                if let Some(args) = args {
                    action(args);
                }
            }
            CommandType::UI(action) => action().unwrap(),
            CommandType::UIWithReturn(action) => {
                let result = action().unwrap();
                println!("Result: {}", result);
            }
        }
    }
}

#[derive(Clone)]
pub struct CliConfig<'a> {
    pub app_name: &'static str,
    pub version: &'static str,
    pub about: &'static str,
    pub commands: Vec<CommandConfig<'a>>,
}

impl<'a> CliConfig<'a> {
    pub fn new(app_name: &'static str, version: &'static str, about: &'static str) -> Self {
        CliConfig {
            app_name,
            version,
            about,
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: CommandConfig<'a>) -> &mut Self {
        self.commands.push(command);
        self
    }

    pub fn add_input(
        &mut self,
        name: &'static str,
        description: &'static str,
        placeholder: &'static str,
        initial_text: &'static str,
        label: &'static str,
    ) {
        let input_action = move || show_input_form(placeholder, initial_text, label);

        let command = CommandConfig::new_ui_with_return(name, description, Arc::new(input_action));

        self.add_command(command);
    }

    pub fn add_text_area(
        &mut self,
        name: &'static str,
        description: &'static str,
        label: &'a str,
        visible_lines: usize,
    ) {
        let text_area = move || text_area(label, visible_lines);

        let command = CommandConfig::new_ui(name, description, Arc::new(text_area));

        self.add_command(command);
    }

    pub fn commands(&self) -> &[CommandConfig] {
        &self.commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_cli_config() {
        let mut cli_config = CliConfig::new("test_app", "1.0", "A test application");

        let mut command = CommandConfig::new_standard(
            "test_cmd",
            "A test command",
            Arc::new(|args| {
                println!("Executing test_cmd with args: {:?}", args);
            }),
        );

        command.add_arg(ArgConfig {
            name: "message",
            help: "A test message".to_string(),
            required: true,
        });

        cli_config.add_command(command);

        assert_eq!(cli_config.app_name, "test_app");
        assert_eq!(cli_config.version, "1.0");
        assert_eq!(cli_config.about, "A test application");
        assert_eq!(cli_config.commands().len(), 1);
    }
}
