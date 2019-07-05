#[macro_use]
extern crate log;

extern crate rogue;

use std::env;

use rogue::{GameClient};
use rogue::file_logger;

use std::panic;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    let game = GameClient::new();
    game.init(
        args
    ).expect("Failed to init game");

    // game.load_map("assets/test.map")
    //     .expect("Failed to load map");

    if let Err(error) = game.run() {
        error!("{}", error);
    }

    Ok(())
}
