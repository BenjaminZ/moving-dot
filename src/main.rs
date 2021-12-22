use ggez::{ContextBuilder, event, GameResult};
use ggez::conf::{NumSamples, WindowSetup};

use moving_dot::{Boarders, TickState};

pub fn main() -> GameResult {
    let window_setup = WindowSetup {
        title: String::from("A Moving dot"),
        samples: NumSamples::Eight,
        vsync: false,
        icon: String::from(""),
        srgb: false,
    };
    let cb = ContextBuilder::new("super_simple", "benz");
    let (ctx, event_loop) = cb.window_setup(window_setup)
                              .build()?;
    let boarders = Boarders{
        x_left: 0.0,
        x_right: 800.0,
        y_left : 0.0,
        y_right: 600.0
    };
    let state = TickState::new(boarders);
    event::run(ctx, event_loop, state)
}
