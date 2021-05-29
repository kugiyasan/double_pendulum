mod pendulum;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::Context;
use ggez::GameResult;
use ggez::{graphics, timer};
use pendulum::DoublePendulum;

const SCREEN_SIZE: (f32, f32) = (400.0, 400.0);
const DESIRED_FPS: u32 = 60;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("double_pendulum", "kugiyasan")
        .window_setup(WindowSetup::default().title("Double Pendulum"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}

struct MainState {
    pendulums: Vec<DoublePendulum>,
}

impl MainState {
    pub fn new() -> GameResult<Self> {
        let s = Self {
            pendulums: vec![DoublePendulum::new()],
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            for p in &mut self.pendulums {
                p.update(ctx)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for p in &mut self.pendulums {
            p.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::C => self.pendulums.push(DoublePendulum::new()),
            KeyCode::R => self.pendulums = vec![DoublePendulum::new()],
            _ => (),
        }
    }
}
