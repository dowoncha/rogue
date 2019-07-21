#![feature(drain_filter)]

#[macro_use]
extern crate log;
extern crate ncurses;
extern crate rand;

use ncurses as nc;

#[macro_use]
extern crate rogue;

use rogue::{
    Entity,
    Component,
    file_logger, 
    EntityManager, 
    SystemManager,
    drop_ncurses, 
    System, 
    InputSystem, 
    RenderSystem, 
    CollisionSystem,
    AttackSystem,
    Rect,
    WalkSystem,
    DamageSystem,
    MoveSystem,
    MapBuilder,
    Chronos,
    EventLogSystem,
    RandomWalkAiSystem,
    Map,
    Janitor
};

use rogue::map::{simple_map_gen, ca_map_gen};
use rogue::components::{self, Position, Input, Render, RenderLayer, Collidable, Walk};

struct Client;

fn main() {
    // start ncurses

    // Get Input and send to Engine

    // Render entities
}