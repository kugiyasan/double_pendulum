mod mainstate;
mod pendulum;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::GameResult;
use mainstate::MainState;
use std::env;

/// The width and the height of the screen at startup
const INITIAL_SCREEN_SIZE: (f32, f32) = (400.0, 400.0);

/// A little struct that helps to parse the command line arguments
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

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Double Pendulum");
    let window_mode = WindowMode::default()
        .dimensions(INITIAL_SCREEN_SIZE.0, INITIAL_SCREEN_SIZE.1)
        .min_dimensions(200.0, 200.0)
        .resizable(true);
    let cb = ggez::ContextBuilder::new("double_pendulum", "kugiyasan")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;

    let config = Config::new(env::args());
    let center = [INITIAL_SCREEN_SIZE.0 / 2.0, INITIAL_SCREEN_SIZE.1 / 2.0];
    let state = MainState::new(config.size, config.show_trail, center)?;
    event::run(ctx, event_loop, state)
}
