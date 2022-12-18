use crate::pendulum::DoublePendulum;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
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
    center: [f32; 2],
}

impl MainState {
    pub fn new(size: usize, show_trail: bool, center: [f32; 2]) -> GameResult<Self> {
        let mut pendulums = Vec::with_capacity(size);
        for _ in 0..size {
            pendulums.push(DoublePendulum::new(center[1]));
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
        while ctx.time.check_update_time(DESIRED_FPS) {
            for p in &mut self.pendulums {
                p.update(DESIRED_FPS)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, Some([0.1, 0.2, 0.3, 1.0].into()));

        for p in &mut self.pendulums {
            p.draw(ctx, &mut canvas, self.center, self.show_trail)?;
        }

        // Draw a white circle in the center of the screen
        let origin = [0.0, 0.0];
        let circle = Mesh::new_circle(ctx, DrawMode::fill(), origin, 10.0, 2.0, Color::WHITE)?;
        canvas.draw(&circle, self.center);

        // Write the fps and the number of pendulums in the top left corner
        let text = graphics::Text::new(format!(
            "FPS: {}\nPendulums count: {}",
            ctx.time.fps().round(),
            self.pendulums.len(),
        ));
        let dest_point = [10.0, 10.0];
        canvas.draw(&text, dest_point);

        canvas.finish(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.center = [width / 2.0, height / 2.0];
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(VirtualKeyCode::C) => self.pendulums.push(DoublePendulum::new(self.center[1])),
            Some(VirtualKeyCode::R) => self.pendulums = vec![DoublePendulum::new(self.center[1])],
            Some(VirtualKeyCode::T) => self.show_trail = !self.show_trail,
            Some(VirtualKeyCode::Q) => ctx.request_quit(),
            _ => (),
        };
        Ok(())
    }
}
