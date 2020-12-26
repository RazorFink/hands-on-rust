#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;
pub const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Wall,
    Floor,
}

struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y:i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }

    }
}
