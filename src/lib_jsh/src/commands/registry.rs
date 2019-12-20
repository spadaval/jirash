use super::api::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

impl CommandRegistry {
    pub fn add_command(&mut self, command: Command) {
        self.commands.insert(command.keyword(), command);
    }
    pub fn get(&self, name: &String) -> &Command {
        self.commands.get(name).unwrap()
    }
    pub fn new() -> CommandRegistry {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }
}
