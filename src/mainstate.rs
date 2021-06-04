use crate::pendulum::DoublePendulum;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, DrawMode, Mesh};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;

/// This value controls the number of physics updates per second
const DESIRED_FPS: u32 = 240;

pub struct MainState {
    /// A vector of every double pendulum on the screen
    pendulums: Vec<DoublePendulum>,
    /// Stores whether the trail of each pendulum should be drawn or not
    ///
    /// Note that the trail is still updated at each frame
    show_trail: bool,
    /// The coordinates of the center of the screen
    center: Point2<f32>,
}

impl MainState {
    pub fn new(size: usize, show_trail: bool, center: Point2<f32>) -> GameResult<Self> {
        let mut pendulums = Vec::with_capacity(size);
        for _ in 0..size {
            pendulums.push(DoublePendulum::new(center.y));
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
        // Update every pendulum `DESIRED_FPS` number of times per second
        while timer::check_update_time(ctx, DESIRED_FPS) {
            for p in &mut self.pendulums {
                p.update(DESIRED_FPS)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for p in &mut self.pendulums {
            p.draw(ctx, self.center, self.show_trail)?;
        }

        // Draw a white circle in the center of the screen
        let origin = Point2::new(0.0, 0.0);
        let circle = Mesh::new_circle(ctx, DrawMode::fill(), origin, 10.0, 2.0, graphics::WHITE)?;
        graphics::draw(ctx, &circle, (self.center,))?;

        // Write the fps and the number of pendulums in the top left corner
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

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.center = Point2::new(width / 2.0, height / 2.0);
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::C => self.pendulums.push(DoublePendulum::new(self.center.y)),
            KeyCode::R => self.pendulums = vec![DoublePendulum::new(self.center.y)],
            KeyCode::T => self.show_trail = !self.show_trail,
            KeyCode::Q => event::quit(ctx),
            _ => (),
        }
    }
}
