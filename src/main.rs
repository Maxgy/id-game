use bracket_lib::prelude::*;

use id_game::MainState;

fn main() {
    let context = BTermBuilder::simple80x50().with_title("id-game").build();
    let gs = MainState::new();

    main_loop(context, gs);
}
