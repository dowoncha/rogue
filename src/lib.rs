#[macro_use]
extern crate log;
extern crate rand;
extern crate shred;
#[macro_use]
extern crate shred_derive;
extern crate specs;

extern crate time;

extern crate sdl2;

pub mod types;
pub mod rogue;
mod character;
mod dungeon;
// mod renderer;

pub use rogue::RogueGame;

#[macro_use] extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(librogue, initlibrogue, PyInit__librogue, |py, m| {

});