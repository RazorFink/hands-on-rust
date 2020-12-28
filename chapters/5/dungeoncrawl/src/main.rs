#![warn(clippy::all, clippy::pedantic)]

mod map;
mod player;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::player::*;
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
        Self {
            map: Map::new(),
            player: Player::new(
                Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
            ),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }

}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello, world!")
        .build()?;

    main_loop(context, State::new())
}
