use bracket_lib::prelude::*;

use legion::prelude::*;

pub enum CurrentState {
    Menu,
    Playing,
    Quitting,
}

pub struct MainState {
    curr_state: CurrentState,
    world: World,
    messages: Vec<String>,
}

impl MainState {
    pub fn new() -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let rects = vec![
            (Rect::with_size(10, 10, 10, 10),),
            (Rect::with_size(50, 5, 20, 5),),
        ];
        world.insert((), rects.into_iter());

        Self {
            curr_state: CurrentState::Menu,
            world,
            messages: vec![String::new()],
        }
    }

    fn menu_state(&mut self, ctx: &mut BTerm) {
        ctx.print(10, 2, "id-game");
        ctx.print(1, 5, "Press the spacebar to start");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.curr_state = CurrentState::Playing,
                _ => (),
            }
        }
    }

    fn play_state(&mut self, ctx: &mut BTerm) {
        let read_query = <(Read<Rect>,)>::query();

        ctx.print(1, 2, "Hello, sailor!");

        for (y, message) in self.messages.iter().enumerate() {
            ctx.print(1, y as i32 + 5, message);
        }

        for (rect,) in read_query.iter_immutable(&self.world) {
            ctx.draw_box(
                rect.x1,
                rect.y2,
                rect.width(),
                rect.height(),
                RGB::from_u8(255, 255, 255),
                RGB::from_u8(0, 0, 0),
            );
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Q => self.curr_state = CurrentState::Quitting,
                VirtualKeyCode::Return => {
                    if !self.messages.last().unwrap().is_empty() {
                        self.messages.push(String::new());
                    }
                }
                VirtualKeyCode::Back => {
                    self.messages.last_mut().unwrap().pop();
                }
                VirtualKeyCode::Space => self.messages.last_mut().unwrap().push(' '),
                _ => self
                    .messages
                    .last_mut()
                    .unwrap()
                    .push(format!("{:?}", key).chars().last().unwrap_or(' ')),
            }
        }
    }

    fn quit_state(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 2, "Are you sure you want to quit? (y/n)");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Y => ctx.quit(),
                VirtualKeyCode::N => self.curr_state = CurrentState::Playing,
                _ => (),
            }
        }
    }
}

impl GameState for MainState {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.curr_state {
            CurrentState::Menu => self.menu_state(ctx),
            CurrentState::Playing => self.play_state(ctx),
            CurrentState::Quitting => self.quit_state(ctx),
        }
    }
}
