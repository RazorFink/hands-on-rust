#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }

}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello, world!")
        .build()?;

    main_loop(context, State::new())
}
