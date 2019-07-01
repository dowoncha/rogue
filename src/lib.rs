#![crate_name = "rogue"]
#![crate_type = "lib"]

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate rand;

extern crate time;

extern crate ncurses;

pub mod errors {
    error_chain! {

    }
}

mod types;
mod character;
mod dungeon;
mod game_state;
mod engine;
mod renderer;

pub use engine::GameClient;


