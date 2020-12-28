#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            WHITE,
            to_cp437('@'),
        );
    }
}
