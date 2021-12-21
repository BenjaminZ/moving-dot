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
    boarders: Boarders,
}

impl TickState {
    pub fn new(boarders: Boarders) -> TickState {
        let dot = Dot {
            x: 400.5,
            y: 300.5,
            radius: 10.0
        };
        TickState {
            dot,
            boarders,
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
            self.dot.radius.clone(),
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
        dot.move_direction(direction, &self.boarders);
    }
}

struct Dot {
    x: f32,
    y: f32,
    radius: f32,
}

pub struct Boarders {
    pub x_left: f32,
    pub x_right: f32,
    pub y_left: f32,
    pub y_right: f32,
}

impl Dot {
    fn move_direction(&mut self, direction: Direction, boarders: &Boarders) {
        match direction {
            Direction::Up => self.y = self.y - 5.0,
            Direction::Down => self.y = self.y + 5.0,
            Direction::Left => self.x = self.x - 5.0,
            Direction::Right => self.x = self.x + 5.0,
        }

        if self.y < boarders.y_left + self.radius {
            self.y = self.radius;
        }

        if self.y > boarders.y_right - self.radius {
            self.y = boarders.y_right - self.radius ;
        }

        if self.x < boarders.x_left + self.radius {
            self.x = self.radius;
        }

        if self.x > boarders.x_right - self.radius {
            self.x = boarders.x_right - self.radius ;
        }
    }
}
