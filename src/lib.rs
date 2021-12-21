use std::thread;
use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use glam::*;

use control::Direction;

mod control;

pub struct TickState {
    dot: Dot,
}

impl TickState {
    pub fn new() -> TickState {
        let dot = Dot {
            x: 400.5,
            y: 300.5
        };
        TickState {
            dot,
        }
    }
}

impl event::EventHandler<ggez::GameError> for TickState {
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
                      _: &mut Context,
                      keycode: KeyCode,
                      _: KeyMods,
                      _: bool) {
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

struct Dot {
    x: f32,
    y: f32,
}
