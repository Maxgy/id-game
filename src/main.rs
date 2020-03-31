use bracket_lib::prelude::*;

use id_game::State;

fn main() {
    let ctx = BTermBuilder::simple(80, 40)
        .unwrap()
        .with_tile_dimensions(16, 16)
        .with_title("id-game")
        .build()
        .unwrap();
    let gs = State::new(80, 40);

    main_loop(ctx, gs).unwrap();
}
