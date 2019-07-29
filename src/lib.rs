#![crate_name = "rogue"]
#![crate_type = "lib"]
#![feature(duration_float)]
#![feature(option_flattening)]
#![feature(vec_remove_item)]
#![feature(let_chains)]
#![recursion_limit = "1024"]

/**
 * Input -> Walk -> Collision -> Move
 */

extern crate backtrace;

// #[macro_use]
// extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate rand;
extern crate rand_distr;

extern crate serde;
extern crate serde_json;

extern crate time;

extern crate ncurses;

extern crate uuid;

use rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cell::RefCell;

#[macro_use]
pub mod components;
pub use components::{Component, ComponentType};

mod entities;
pub use entities::{Entity, EntityManager};

pub mod map;
mod types;
pub mod monsters;
pub mod items;
pub mod renderer;

#[macro_use]
pub mod systems;

pub use types::*;
pub use map::{Map, MapBuilder};
pub mod file_logger;