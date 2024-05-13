use std::sync::Arc;

#[derive(Clone)]
pub struct ArgConfig {
    pub name: &'static str,
    pub help: String,
    pub required: bool,
}

#[derive(Clone)]
pub struct CommandConfig<'a> {
    pub name: &'static str,
    pub description: &'static str,
    pub args: Vec<ArgConfig>,
    pub action: Arc<dyn Fn(&[String]) + Send + Sync + 'a>,
}

impl<'a> CommandConfig<'a> {
    pub fn new(
        name: &'static str,
        description: &'static str,
        action: Arc<dyn Fn(&[String]) + Send + Sync + 'a>,
    ) -> Self {
        CommandConfig {
            name,
            description,
            args: Vec::new(),
            action,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn add_arg(&mut self, arg: ArgConfig) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn args(&self) -> &[ArgConfig] {
        &self.args
    }

    // Executes the action associated with this command
    pub fn execute_action(&self, args: &[String]) {
        (self.action)(args);
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

    pub fn commands(&self) -> &[CommandConfig] {
        &self.commands
    }
}
