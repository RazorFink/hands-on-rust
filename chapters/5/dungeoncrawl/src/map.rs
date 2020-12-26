#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;
pub const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Wall,
    Floor
}

