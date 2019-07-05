#[macro_use]
extern crate log;

extern crate rogue;

use std::env;

use rogue::{GameClient};
use rogue::file_logger;

use rogue::errors::*;

fn main() -> std::result::Result<(), Box<std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    file_logger::init()
        .expect("Failed to init file logger");

    let mut game = GameClient::new();
    game.init(
        args
    );

    game.load_map("assets/test.map");
    game.run();

    Ok(())
}
