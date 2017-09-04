#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rogue;

use rogue::{Game};

use std::env;

use rogue::errors::*;

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Handle error chain
    if let Err(ref e) = run() {
        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // let args: Vec<_> = env::args().collect();

    let mut game = Game::new();
    game.init();
    game.run();

    Ok(())
}
