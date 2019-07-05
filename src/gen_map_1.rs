// https://www.reddit.com/r/gamedev/comments/1dlwc4/procedural_dungeon_generation_algorithm_explained/
// https://www.gamasutra.com/blogs/AAdonaac/20150903/252889/Procedural_Dungeon_Generation_Algorithm.php

// 1. Choose number of cells to gen (e.g. 150)
// 2. For each cell, spawn a rectangle of random width and length within some radius
use rand::prelude::*;
use rand_distr::StandardNormal;

struct Cell {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

fn gen_cells(
    num: i32,
    radius: f32
) -> Vec<Cell> {
    let mut cells = Vec::new();

    for i in 0..num {
        cells.push(Cell { 
            x: 0,
            y: 0,
            width: 0,
            height: 0
        });
    }

    cells
}

// https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
fn get_random_point_in_circle(radius: f64) -> (f64, f64) {
    let t: f64 = 2.0 * std::f64::consts::PI * thread_rng().sample::<f64, StandardNormal>(StandardNormal);
    let u: f64 = thread_rng().sample::<f64, StandardNormal>(StandardNormal) + thread_rng().sample::<f64, StandardNormal>(StandardNormal);
    let r;
    if u > 1.0 {
        r = 2.0 - u;
    } else {
        r = u;
    }

    (radius * r * t.cos(), radius * r * t.sin())
}

#[test]
fn it_should_gen_number_of_cells() {
    let num_cells = 150;
    let radius = 50.0;
    let cells = gen_cells(num_cells, radius);

    assert_eq!(cells.len(), 150);
}

