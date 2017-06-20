#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rogue;

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Initialize graphics basics

    // Interpret command line arguments

    let mut game = rogue::RogueGame::new();
    game.init();
    game.run();
}