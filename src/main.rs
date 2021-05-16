mod pendulum;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::GameResult;
use pendulum::DoublePendulum;

const SCREEN_SIZE: (f32, f32) = (400.0, 400.0);

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("double_pendulum", "kugiyasan")
        .window_setup(WindowSetup::default().title("Double Pendulum"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut DoublePendulum::new()?;
    event::run(ctx, event_loop, state)
}
