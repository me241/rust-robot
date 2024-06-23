extern crate noise;

use noise::{NoiseFn, Perlin};
use serde::{Serialize, Deserialize};
use std::fmt;

const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Scientific,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    grid: [[Cell; MAP_WIDTH]; MAP_HEIGHT],
}

impl Map {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new();
        let mut grid = [[Cell::Empty; MAP_WIDTH]; MAP_HEIGHT];
        
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0, seed as f64]);
                grid[y][x] = if noise_value > 0.2 {
                    Cell::Obstacle
                } else {
                    Cell::Empty
                };
            }
        }

        // Add resources randomly
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            grid[rng.gen_range(0..MAP_HEIGHT)][rng.gen_range(0..MAP_WIDTH)] = Cell::Energy;
            grid[rng.gen_range(0..MAP_HEIGHT)][rng.gen_range(0..MAP_WIDTH)] = Cell::Mineral;
            grid[rng.gen_range(0..MAP_HEIGHT)][rng.gen_range(0..MAP_WIDTH)] = Cell::Scientific;
        }

        Map { grid }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                print!(
                    "{}",
                    match cell {
                        Cell::Empty => ".",
                        Cell::Obstacle => "#",
                        Cell::Energy => "E",
                        Cell::Mineral => "M",
                        Cell::Scientific => "S",
                    }
                );
            }
            println!();
        }
    }
}
