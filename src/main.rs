#[macro_use]
extern crate log;
extern crate env_logger;

extern crate cpython;

use cpython::{Python, PyDict, PyResult};

extern crate rogue;

extern crate sdl2;

use std::env;

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // TODO: Handle command line arguments through clap

    let args: Vec<_> = env::args().collect();
    
    let mut rogue = rogue::RogueGame::init().unwrap();
    rogue.run();
}