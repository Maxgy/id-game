use bracket_lib::prelude::*;

use kingslayer::Lexer;

use legion::prelude::*;

use crate::{components::Room, input::Parser};

const WHITE: (u8, u8, u8) = (255, 255, 255);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CurrentState {
    Menu,
    Playing,
}

pub struct State {
    curr_state: CurrentState,
    world: World,
    window_size: (u32, u32),
    tic: u8,
    offset: (i32, i32),
    mouse: Point,
    mouse_pressed: bool,
    mouse_released: bool,
    cursor: String,
    messages: Vec<String>,
}

impl State {
    pub fn new(w: u32, h: u32) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let rects = vec![(Room::new(Rect::with_size(10, 10, 10, 10)),)];
        world.insert((), rects.into_iter());

        Self {
            curr_state: CurrentState::Menu,
            world,
            window_size: (w, h),
            tic: 0,
            offset: (0, 0),
            mouse: Point::new(0, 0),
            mouse_pressed: false,
            mouse_released: false,
            cursor: String::from("<"),
            messages: vec![String::new()],
        }
    }

    fn menu_state(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(self.window_size.1 as i32 / 2 - 1, "id-game");
        ctx.print_centered(
            self.window_size.1 as i32 / 2 + 1,
            "Press the spacebar to start",
        );

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.curr_state = CurrentState::Playing;
        }
    }

    fn play_state(&mut self, ctx: &mut BTerm) {
        self.tic += 4;
        if self.tic > 99 {
            self.tic = 0;
        }

        self.mouse = ctx.mouse_point();

        if ctx.left_click {
            if self.mouse_pressed {
                self.mouse_released = true;
            }
            self.mouse_pressed = !self.mouse_pressed;
        }

        // Render custom mouse cursor
        ctx.print_color(
            self.mouse.x,
            self.mouse.y,
            RGB::named((0, 155 + self.tic, 0)),
            RGB::new(),
            &self.cursor,
        );

        ctx.draw_bar_vertical(50, 0, 50, 1, 1, RGB::named(WHITE), RGB::named(WHITE));

        // Print messages
        for (y, message) in self.messages.iter().enumerate() {
            ctx.print(1, y as i32 + 5, message);
        }

        self.print_rooms(ctx);

        self.key_input(ctx);

        self.mouse_released = false;
    }

    fn key_input(&mut self, ctx: &BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => {
                    let last = if let Some(last) = self.messages.last() {
                        last.clone()
                    } else {
                        String::new()
                    };
                    if !last.is_empty() {
                        self.messages.push(Parser::parse(Lexer::lex(&last)));

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

    fn print_rooms(&mut self, ctx: &mut BTerm) {
        let read_query = <(Read<Room>,)>::query();

        for (room,) in read_query.iter_immutable(&self.world) {
            ctx.draw_box(
                room.rect().x1 + self.offset.0,
                room.rect().y2 + self.offset.1,
                room.rect().width(),
                room.rect().height(),
                RGB::named(WHITE),
                RGB::new(),
            );
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.curr_state {
            CurrentState::Menu => self.menu_state(ctx),
            CurrentState::Playing => self.play_state(ctx),
        }
    }
}
