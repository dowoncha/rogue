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

    fn is_command_event(&self, event: &str) -> bool {
        // Check if event should generate  a command 
        // If the event is a key input check if it is binded in the config
        // If it is, create the binded command
        let mut args = event.split_whitespace().collect::<Vec<&str>>();

        if (args.len() > 0) {
            match args[0] {
                "key" => {
                    // let keycode = args[1].parse::<i32>().unwrap();
                    return true;
                }
                _ => {
                    // debug!("Unrecognized command {}", command);
                    // self.renderer.mvprintw(1, 1, &format!("Unrecognized command: {}", command));
                }
            };
        }

        return false;
    }

    pub fn get_key_event_input(&self, event: &str) -> Option<i32> {
        let mut args = event.split_whitespace().collect::<Vec<&str>>();

        if (args.len() > 0) {
            match args[0] {
                "key" => {
                    let keycode = args[1].parse::<i32>().unwrap();
                    Some(keycode)
                }
                _ => {
                    warn!("Not a key event {}", event);
                    None
                    // self.renderer.mvprintw(1, 1, &format!("Unrecognized command: {}", command));
                }
            }
        } else {
            None
        }
    }

    pub fn handle_input(&mut self, input: i32) -> Option<Command> {
        match input {
            119 => {
                // 'w'
                // let player_x = self.player.x;
                // let player_y = self.player.y;
                // self.player.set_y(player_y - 1);

                // return Some(Command::MoveEntity { 
                //     entity_id: "player",
                //     x: player_x, 
                //     y: player_y - 1,
                // });
            },
            115 => {
                // 'd'
                // let player_y = self.player.y;

                // self.player.set_y(player_y + 1);
            },
            100 => {
                // 'd'
                // let player_x = self.player.x;
                // self.player.set_x(player_x + 1);
            },
            97 => {
                // 'a'
                // let player_x = self.player.x;

                // self.player.set_x(player_x - 1);
            },
            113 => {
                // 'q'
                // self.event_sender.send("quit".to_string());
                // return Some(Command::Quit);
            },
            _ => {}
        }

        return None
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