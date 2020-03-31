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

add_wasm_support!();

pub struct State {
    curr_state: CurrentState,
    world: World,
    window_size: (u32, u32),
    tic: u8,
    offset: (i32, i32),
    mouse: Point,
    mouse_click: Option<(usize, bool)>,
    mouse_pressed: (usize, bool),
    cursor: String,
    messages: Vec<String>,
}

impl State {
    pub fn new(w: u32, h: u32) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let rooms = vec![(Room::new(Rect::with_size(10, 10, 10, 10)),)];
        world.insert((), rooms.into_iter());

        Self {
            curr_state: CurrentState::Menu,
            world,
            window_size: (w, h),
            tic: 0,
            offset: (0, 0),
            mouse: Point::new(0, 0),
            mouse_click: None,
            mouse_pressed: (0, false),
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
        let mut input = INPUT.lock();

        input.for_each_message(|event| match event {
            BEvent::MouseClick { button, pressed } => self.mouse_click = Some((button, pressed)),
            BEvent::MouseButtonUp { button } => self.mouse_pressed = (button, false),
            BEvent::MouseButtonDown { button } => self.mouse_pressed = (button, true),
            _ => (),
        });

        self.tic += 4;
        if self.tic > 99 {
            self.tic = 0;
        }

        self.mouse = ctx.mouse_point();

        ctx.print_color(
            self.mouse.x,
            self.mouse.y,
            RGB::named((0, 155 + self.tic, 0)),
            RGB::new(),
            &self.cursor,
        );

        ctx.draw_bar_vertical(50, 0, 50, 1, 1, RGB::named(WHITE), RGB::named(WHITE));

        self.print_messages(ctx);

        self.print_rooms(ctx);

        self.key_input(ctx);

        self.mouse_click = None;
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

    fn print_messages(&mut self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;
        let mut line_len = 0;
        for message in self.messages.iter() {
            for c in message.chars() {
                if c == ' ' {
                    line_len = x;
                }
                if line_len > 15 {
                    y += 1;
                    x = 0;
                    line_len = 0;
                } else {
                    x += 1;
                }
                ctx.print(52 + x as i32, y as i32 + 5, &c.to_string());
            }
            x = 0;
            y += 2;
            line_len = 0;
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
