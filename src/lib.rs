#![crate_name = "rogue"]
#![crate_type = "lib"]

#![feature(duration_float)]

#![recursion_limit = "1024"]

extern crate backtrace;

// #[macro_use]
// extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate rand;
extern crate rand_distr;

extern crate time;

extern crate ncurses;

mod action;
mod entity;
mod gen_map_1;
mod command_manager;
mod config_manager;
mod input_manager;
pub mod file_logger;
mod types;
mod map;
mod character;
mod dungeon;
mod engine;
mod renderer;
mod client;

pub use client::GameClient;
