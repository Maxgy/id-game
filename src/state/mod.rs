#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use bracket_lib::prelude::*;

use kingslayer::Lexer;

use legion::prelude::*;

use crate::{
    components::{GameCell, Id, Room},
    input::Parser,
    types::Clock,
};

const LIGHT_GRAY: (u8, u8, u8) = (170, 170, 170);
const GREEN: (u8, u8, u8) = (0, 170, 0);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CurrentState {
    Menu,
    Playing,
    Quitting,
}

add_wasm_support!();

pub struct State {
    curr_state: CurrentState,
    world: World,
    schedule: Schedule,
    window_size: (u32, u32),
    tic: u8,
    dt: f32,
    #[cfg(not(target_arch = "wasm32"))]
    instant: Instant,
    mouse: Point,
    mouse_click: Option<(usize, bool)>,
    mouse_pressed: (usize, bool, bool),
    cursor: String,
    messages: Vec<String>,
    clock: Clock,
    tab_shown: bool,
}

impl State {
    pub fn new(w: u32, h: u32) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let rooms = vec![(Room::new(Rect::with_size(14, 14, 6, 6)),)];
        world.insert((), rooms.into_iter());

        let id = vec![(
            GameCell::new(Point::new(17, 17), '@', RGB::named(GREEN)),
            Id {},
        )];
        world.insert((), id.into_iter());

        let schedule = Schedule::builder().flush().build();

        Self {
            curr_state: CurrentState::Menu,
            world,
            schedule,
            window_size: (w, h),
            dt: 0.016,
            #[cfg(not(target_arch = "wasm32"))]
            instant: Instant::now(),
            tic: 0,
            mouse: Point::new(0, 0),
            mouse_click: None,
            mouse_pressed: (0, false, false),
            cursor: String::from("<"),
            messages: vec![String::new()],
            clock: Clock::new(),
            tab_shown: false,
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
        self.schedule.execute(&mut self.world);

        self.clock.update(self.dt);

        self.render_ui(ctx);

        ctx.print_color(
            self.mouse.x,
            self.mouse.y,
            RGB::named((0, 155 + self.tic, 0)),
            RGB::named(WHITE),
            &self.cursor,
        );

        self.render(ctx);

        self.key_input(ctx);
    }

    fn key_input(&mut self, ctx: &BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Tab => self.tab_shown = !self.tab_shown,
                VirtualKeyCode::End => self.curr_state = CurrentState::Quitting,
                _ => self.push_message_char(key),
            }
        }
    }

    fn push_message_char(&mut self, key: VirtualKeyCode) {
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

    fn render_ui(&self, ctx: &mut BTerm) {
        for x in 0..self.window_size.0 {
            for y in 0..self.window_size.1 - 1 {
                ctx.print_color(
                    x as i32,
                    y as i32,
                    RGB::named(LIGHT_GRAY),
                    RGB::named(WHITE),
                    ".",
                )
            }
        }

        if self.tab_shown {
            ctx.draw_hollow_box(
                self.window_size.0 as i32 - self.window_size.0 as i32 / 4 - 1,
                0,
                self.window_size.0 as i32 / 4,
                self.window_size.1 as i32 - 1,
                RGB::new(),
                RGB::named(WHITE),
            );

            self.print_messages(ctx);
        } else {
            ctx.print_color(
                self.window_size.0 as i32 - 10,
                2,
                RGB::new(),
                RGB::named(WHITE),
                "Press TAB",
            );
        }
        ctx.draw_hollow_box(
            0,
            0,
            self.window_size.0 as i32 - 1,
            self.window_size.1 as i32 - 1,
            RGB::new(),
            RGB::named(WHITE),
        );

        ctx.print_color(2, 2, RGB::new(), RGB::named(WHITE), self.clock.print());
    }

    fn print_messages(&self, ctx: &mut BTerm) {
        let mut y = 1;
        let mut x = 0;
        let mut line_len = 0;
        for message in self.messages.iter() {
            for c in message.chars() {
                if c == ' ' {
                    line_len = x;
                }
                if line_len > self.window_size.0 / 4 - 7 {
                    y += 1;
                    x = 0;
                    line_len = 0;
                } else {
                    x += 1;
                }
                ctx.print_color(
                    self.window_size.0 as i32 - self.window_size.0 as i32 / 4 + x as i32,
                    y as i32,
                    RGB::new(),
                    RGB::named(WHITE),
                    &c.to_string(),
                );
            }
            x = 0;
            y += 2;
            line_len = 0;
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        let read_query = <(Read<Room>,)>::query();

        for (room,) in read_query.iter_immutable(&self.world) {
            ctx.draw_hollow_box(
                room.rect().x1,
                room.rect().y1,
                room.rect().width(),
                room.rect().height(),
                RGB::new(),
                RGB::named(WHITE),
            );
        }

        let read_query = <(Read<GameCell>,)>::query();

        for (cell,) in read_query.iter_immutable(&self.world) {
            ctx.print_color(
                cell.x(),
                cell.y(),
                if self.mouse.x == cell.x() && self.mouse.y == cell.y() {
                    cell.color_bright()
                } else {
                    cell.color()
                },
                cell.bg_color(),
                cell.symbol(),
            );
        }
    }

    fn quit_state(&mut self, ctx: &mut BTerm) {
        ctx.print(5, 5, "Are you sure you want to quit? (y/n)");

        if let Some(VirtualKeyCode::Y) = ctx.key {
            ctx.quit();
        } else if let Some(VirtualKeyCode::N) = ctx.key {
            self.curr_state = CurrentState::Playing;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn update_dt(&self) {}
    #[cfg(not(target_arch = "wasm32"))]
    fn update_dt(&mut self) {
        self.dt = Instant::now().duration_since(self.instant).as_secs_f32();
        self.instant = Instant::now();
    }

    #[cfg(target_arch = "wasm32")]
    fn get_input(&mut self) {
        self.mouse_pressed.2 = false;

        let mut input = INPUT.lock();

        input.for_each_message(|event| match event {
            BEvent::MouseClick { button, pressed } => self.mouse_click = Some((button, pressed)),
            BEvent::MouseButtonUp { button } => {
                self.mouse_pressed = (button, false, self.mouse_pressed.1)
            }
            BEvent::MouseButtonDown { button } => {
                self.mouse_pressed = (button, true, self.mouse_pressed.1)
            }
            _ => (),
        });

        if !self.mouse_pressed.1 && self.mouse_pressed.2 {
            self.mouse_click = Some((self.mouse_pressed.0, false))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_input(&mut self) {
        let mut input = INPUT.lock();

        input.for_each_message(|event| match event {
            BEvent::MouseClick { button, pressed } => self.mouse_click = Some((button, pressed)),
            BEvent::MouseButtonUp { button } => self.mouse_pressed = (button, false, false),
            BEvent::MouseButtonDown { button } => self.mouse_pressed = (button, true, false),
            _ => (),
        });
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.update_dt();

        ctx.cls();

        self.get_input();

        self.tic += 4;
        if self.tic > 99 {
            self.tic = 0;
        }

        self.mouse = ctx.mouse_point();

        match self.curr_state {
            CurrentState::Menu => self.menu_state(ctx),
            CurrentState::Playing => self.play_state(ctx),
            CurrentState::Quitting => self.quit_state(ctx),
        }

        self.mouse_click = None;
    }
}
