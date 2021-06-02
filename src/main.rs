mod mainstate;
mod pendulum;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::nalgebra::Point2;
use ggez::GameResult;
use mainstate::MainState;
use std::env;

const SCREEN_SIZE: (f32, f32) = (400.0, 400.0);
const CENTER: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("double_pendulum", "kugiyasan")
        .window_setup(WindowSetup::default().title("Double Pendulum"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?;

    let config = Config::new(env::args());
    let center = Point2::new(CENTER.0, CENTER.1);
    let state = &mut MainState::new(config.size, config.show_trail, center)?;
    event::run(ctx, event_loop, state)
}

struct Config {
    size: usize,
    show_trail: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Self {
        args.next();

        let size = args.next().unwrap_or(String::new()).parse().unwrap_or(1);
        let show_trail = args.next().unwrap_or(String::new()) == "true";

        Self { size, show_trail }
    }
}
