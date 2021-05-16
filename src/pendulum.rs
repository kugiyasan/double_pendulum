use std::collections::VecDeque;

use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use ggez::{event, timer};

const GRAVITY: f32 = 9.8;
const CENTER: (f32, f32) = (200.0, 200.0);

// https://en.wikipedia.org/wiki/Double_pendulum#Lagrangian
// https://en.wikipedia.org/wiki/Euler_method
// https://www.myphysicslab.com/pendulum/double-pendulum-en.html

struct Pendulum {
    mass: f32,
    radius: f32,
    theta: f32,
    speed: f32,
    momentum: f32,
}

impl Pendulum {
    fn new() -> Self {
        Self {
            mass: 1.0,
            radius: 100.0,
            theta: 1.0,
            speed: 0.1,
            momentum: 10.0,
        }
    }

    /// Returns the x coordinate of the tip of the rod
    fn x(&self) -> f32 {
        self.radius * self.theta.sin()
    }

    /// Returns the y coordinate of the tip of the rod
    fn y(&self) -> f32 {
        -self.radius * self.theta.cos()
    }
}

pub struct DoublePendulum {
    pendulum1: Pendulum,
    pendulum2: Pendulum,
    trail: VecDeque<Point2<f32>>,
}

impl DoublePendulum {
    /// Most equations for the double pendulum comes from the wiki page:
    /// https://en.wikipedia.org/wiki/Double_pendulum
    pub fn new() -> GameResult<Self> {
        let s = Self {
            pendulum1: Pendulum::new(),
            pendulum2: Pendulum::new(),
            trail: VecDeque::new(),
        };
        Ok(s)
    }

    /// https://wikimedia.org/api/rest_v1/media/math/render/svg/ea30dfe9ba779902cca5f518a71567407e4974ce
    fn update_speed_1(&mut self) {
        let p1 = &self.pendulum1;
        let p2 = &self.pendulum2;

        let cos = (p1.theta - p2.theta).cos();
        let num = 12.0 * p1.momentum - 18.0 * cos * p2.momentum;

        let l = p1.radius;
        let denom = p1.mass * l * l * (16.0 - 9.0 * cos * cos);

        self.pendulum1.speed = num / denom;
    }

    /// https://wikimedia.org/api/rest_v1/media/math/render/svg/ea30dfe9ba779902cca5f518a71567407e4974ce
    fn update_speed_2(&mut self) {
        let p1 = &self.pendulum1;
        let p2 = &self.pendulum2;

        let cos = (p1.theta - p2.theta).cos();
        let num = 48.0 * p2.momentum - 18.0 * cos * p1.momentum;

        let l = p1.radius;
        let denom = p1.mass * l * l * (16.0 - 9.0 * cos * cos);

        self.pendulum1.speed = num / denom;
    }

    /// https://wikimedia.org/api/rest_v1/media/math/render/svg/d8e7f78e4cef6b9b0b46bf5f5050c72a7b1ed725
    fn update_momentum_1(&mut self) {
        let p1 = &self.pendulum1;
        let p2 = &self.pendulum2;

        let m = p1.mass;
        let l = p1.radius;
        let a = p1.speed * p2.speed * (p1.theta - p2.theta).sin();
        let b = 3.0 * GRAVITY / l * p1.theta.sin();

        self.pendulum1.momentum += -0.5 * m * l * l * (a + b);
    }

    /// https://wikimedia.org/api/rest_v1/media/math/render/svg/d8e7f78e4cef6b9b0b46bf5f5050c72a7b1ed725
    fn update_momentum_2(&mut self) {
        let p1 = &self.pendulum1;
        let p2 = &self.pendulum2;

        let m = p1.mass;
        let l = p1.radius;
        let a = -p1.speed * p2.speed * (p1.theta - p2.theta).sin();
        let b = GRAVITY / l * p2.theta.sin();

        self.pendulum2.momentum += -0.5 * m * l * l * (a + b);
    }

    fn forward(&mut self) {
        let step = 1.0 / 60.0;

        // https://en.wikipedia.org/wiki/Double_pendulum#Lagrangian
        // L = kinetic_energy - potential_energy

        // https://en.wikipedia.org/wiki/Euler_method
        // self.pendulum1.theta += step * self.momentum_der_theta_1();
        // self.pendulum2.theta += step * self.momentum_der_theta_2();
        // self.pendulum1.theta += step;
        // self.pendulum2.theta += step;

        self.update_momentum_1();
        self.update_momentum_2();
        self.update_speed_1();
        self.update_speed_2();

        self.pendulum1.theta += step * self.pendulum1.speed;
        self.pendulum2.theta += step * self.pendulum2.speed;
    }
}

impl event::EventHandler for DoublePendulum {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.forward();

            let x = self.pendulum1.x() + self.pendulum2.x();
            let y = self.pendulum1.y() + self.pendulum2.y();
            let point = Point2::new(x, y);
            self.trail.push_back(point);
            if self.trail.len() > 100 {
                self.trail.pop_front();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let x_1 = self.pendulum1.x();
        let y_1 = self.pendulum1.y();
        let x_2 = x_1 + self.pendulum2.x();
        let y_2 = y_1 + self.pendulum2.y();

        let origin = Point2::new(0.0, 0.0);
        let p1 = Point2::new(x_1, y_1);
        let p2 = Point2::new(x_2, y_2);

        let line = Mesh::new_line(ctx, &[origin, p1, p2], 2.0, graphics::WHITE)?;

        let circle_1 = Mesh::new_circle(ctx, DrawMode::fill(), origin, 10.0, 2.0, graphics::WHITE)?;
        let circle_2 = Mesh::new_circle(ctx, DrawMode::fill(), p1, 10.0, 2.0, graphics::WHITE)?;
        let circle_3 = Mesh::new_circle(ctx, DrawMode::fill(), p2, 10.0, 2.0, graphics::WHITE)?;

        let center = Point2::new(CENTER.0, CENTER.1);
        graphics::draw(ctx, &line, (center,))?;
        graphics::draw(ctx, &circle_1, (center,))?;
        graphics::draw(ctx, &circle_2, (center,))?;
        graphics::draw(ctx, &circle_3, (center,))?;

        let trail = Mesh::new_line(
            ctx,
            self.trail.make_contiguous(),
            2.0,
            [0.1, 0.5, 0.1, 1.0].into(),
        )?;
        graphics::draw(ctx, &trail, (center,))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
