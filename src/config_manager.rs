/**
 * Config manager
 * 
 * Loads configurations from launch options, and config file
 * 
 * A config file is basically a script of commands
 */

enum Key {
    W = 87,
    a = 97,
    d = 100,
    s = 115,
    w = 119,
}

pub struct ConfigManager {
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {}
    }

    fn get_keybinds(&self) -> Keybinds {
        Keybinds {

        }
    }

    fn execute(&self, filename: &str) {

    }
}

struct Keybinds {
}

impl Keybinds {
    pub fn get(&self, key: Key) -> Option<&str> {
        let bind = match key {
            Key::w => "+forward",
            Key::s => "+back",
            Key::a => "+moveleft",
            Key::d => "+moveright",
            _ => ""
        };

        if (bind.is_empty()) {
            None
        } else {
            Some(bind)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_have_default_keybinds() {
        let cm = ConfigManager::new();

        let keybinds = cm.get_keybinds();

        assert_eq!(keybinds.get(Key::w), Some("+forward"));
        assert_eq!(keybinds.get(Key::s), Some("+back"));
        assert_eq!(keybinds.get(Key::a), Some("+moveleft"));
        assert_eq!(keybinds.get(Key::d), Some("+moveright"));
    }

    #[test]
    fn it_should_load_config_from_file() {
        let cm = ConfigManager::new();

        cm.execute("assets/config.cfg");
    }
}