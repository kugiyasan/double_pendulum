use crate::pendulum::DoublePendulum;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, DrawMode, Mesh};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;

const DESIRED_FPS: u32 = 60;

pub struct MainState {
    pendulums: Vec<DoublePendulum>,
    show_trail: bool,
    center: Point2<f32>,
}

impl MainState {
    pub fn new(size: usize, show_trail: bool, center: Point2<f32>) -> GameResult<Self> {
        let mut pendulums = Vec::with_capacity(size);
        for _ in 0..size {
            pendulums.push(DoublePendulum::new());
        }
        let s = Self {
            pendulums,
            show_trail,
            center,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            for p in &mut self.pendulums {
                p.update()?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for p in &mut self.pendulums {
            p.draw(ctx, self.center, self.show_trail)?;
        }

        let origin = Point2::new(0.0, 0.0);
        let circle = Mesh::new_circle(ctx, DrawMode::fill(), origin, 10.0, 2.0, graphics::WHITE)?;
        graphics::draw(ctx, &circle, (self.center,))?;

        let text = graphics::Text::new(format!(
            "FPS: {}\nPendulums count: {}",
            timer::fps(ctx).round(),
            self.pendulums.len(),
        ));
        let dest_point = Point2::new(10.0, 10.0);
        graphics::draw(ctx, &text, (dest_point,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::C => self.pendulums.push(DoublePendulum::new()),
            KeyCode::R => self.pendulums = vec![DoublePendulum::new()],
            KeyCode::T => self.show_trail = !self.show_trail,
            KeyCode::Q => event::quit(ctx),
            _ => (),
        }
    }
}
