#[macro_use]
extern crate log;
extern crate rand;
// #[macro_use]
extern crate specs;

extern crate ncurses;

pub mod types;
mod rogue;
mod character;
pub use rogue::RogueGame;