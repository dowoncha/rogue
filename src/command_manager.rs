use std::fs::File;
use std::io::prelude::*;

pub enum Command {
    MoveEntity {
        entity_id: &'static str,
        x: i32,
        y: i32
    },
    Quit
}

impl Command {
}

pub struct CommandManager {
    command_history: Vec<Command>
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            command_history: Vec::new()
        }
    }

    pub fn init(&self) -> std::io::Result<()> {
        // Load user's config
        let config_filename = "assets/config.cfg";
        let mut config_file = File::open(config_filename)?;

        let mut config = String::new();

        config_file.read_to_string(&mut config);

        Ok(())
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn it_should_check_if_an_event_is_command() {
    let command_event = "key 150";

    let cm = CommandManager::new();
    let event_is_command = cm.is_command_event(command_event);

    assert!(event_is_command);
}

#[test]
fn it_should_create_moveforward_command() {
    let command_event = "key 119";

    let cm = CommandManager::new();
    // cm.load_config("assets/config.cfg");
}

#[test]
fn it_should_execute_input_command() {

}

}