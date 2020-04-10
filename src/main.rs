use bracket_lib::prelude::*;

use id_game::State;

fn main() -> BError {
    let ctx = BTermBuilder::simple(60, 30)?
        .with_tile_dimensions(20, 20)
        .with_fullscreen(true)
        .with_fps_cap(60.0)
        .with_advanced_input(true)
        .with_title("id-game")
        .build()?;
    let gs = State::new(60, 30);

    main_loop(ctx, gs)
}
