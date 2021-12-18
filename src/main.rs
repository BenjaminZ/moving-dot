use std::thread;
use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::conf::{NumSamples, WindowSetup};
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use glam::*;

struct Dot {
    x: f32,
    y: f32,
}

struct MyState {
    dot: Dot,
}

impl MyState {
    fn new() -> MyState {
        let dot = Dot {
            x: 400.5,
            y: 300.5
        };
        MyState {
            dot,
        }
    }
}

impl event::EventHandler<ggez::GameError> for MyState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        thread::sleep(Duration::from_millis(5));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            0.1,
            Color::WHITE,
        )?;

        let dot = &self.dot;
        graphics::draw(ctx, &circle, (Vec2::new(dot.x, dot.y), ))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self,
                      _ctx: &mut Context,
                      keycode: KeyCode,
                      _keymods: KeyMods,
                      _repeat: bool) {
        let direction = Direction::from(&keycode);
        if let None = direction {
            return;
        }

        let dot = &mut self.dot;
        let direction = direction.unwrap();
        match direction {
            Direction::Up => dot.y = dot.y - 5.0,
            Direction::Down => dot.y = dot.y % 600.0 + 5.0,
            Direction::Left => dot.x = dot.x - 5.0,
            Direction::Right => dot.x = dot.x % 800.0 + 5.0,
        }

        if dot.y < 0.0 {
            dot.y = dot.y + 600.0;
        }

        if dot.x < 0.0 {
            dot.x = dot.x + 800.0;
        }
    }
}

pub fn main() -> GameResult {
    run()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(key: &KeyCode) -> Option<Direction> {
        let input = match key {
            KeyCode::W => Some(Direction::Up),
            KeyCode::Up => Some(Direction::Up),
            KeyCode::S => Some(Direction::Down),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::A => Some(Direction::Left),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::D => Some(Direction::Right),
            KeyCode::Right => Some(Direction::Right),
            _ => None
        };

        input
    }
}

fn run() -> GameResult {
    let window_setup = WindowSetup {
        title: String::from("A Moving dot"),
        samples: NumSamples::Eight,
        vsync: false,
        icon: String::from(""),
        srgb: false,
    };
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.window_setup(window_setup)
                              .build()?;
    let state = MyState::new();
    event::run(ctx, event_loop, state)
}
