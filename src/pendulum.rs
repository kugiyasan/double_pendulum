use std::collections::VecDeque;

use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use rand::Rng;
use std::f32::consts::PI;

const GRAVITY: f32 = 1.0;
const CENTER: (f32, f32) = (200.0, 200.0);

// Useful resources:
// https://en.wikipedia.org/wiki/Double_pendulum#Lagrangian
// https://en.wikipedia.org/wiki/Euler_method
// https://www.myphysicslab.com/pendulum/double-pendulum-en.html

struct Pendulum {
    mass: f32,
    radius: f32,
    theta: f32,
    speed: f32,
}

impl Pendulum {
    fn new(mass: f32, radius: f32, theta: f32, speed: f32) -> Self {
        Self {
            mass,
            radius,
            theta,
            speed,
        }
    }

    /// Returns the x coordinate of the tip of the rod
    fn x(&self) -> f32 {
        self.radius * self.theta.sin()
    }

    /// Returns the y coordinate of the tip of the rod
    fn y(&self) -> f32 {
        self.radius * self.theta.cos()
    }
}

pub struct DoublePendulum {
    p1: Pendulum,
    p2: Pendulum,
    trail: VecDeque<Point2<f32>>,
    color: graphics::Color,
}

impl DoublePendulum {
    pub fn new() -> Self {
        let length = 200.0 / 2.0;
        let mut rng = rand::thread_rng();

        // Spawn the double pendulum straight in the top half with no initial speed
        let m1 = rng.gen_range(2.0..5.0);
        let m2 = rng.gen_range(2.0..5.0);
        let radius = rng.gen_range(0.0..50.0);
        let theta = rng.gen_range(0.0..PI) + PI / 2.0;

        let r = rng.gen_range(0.0..=1.0);
        let g = rng.gen_range(0.0..=1.0);
        let b = rng.gen_range(0.0..=1.0);

        Self {
            p1: Pendulum::new(m1, length + radius, theta, 0.0),
            p2: Pendulum::new(m2, length - radius, theta, 0.0),
            trail: VecDeque::new(),
            color: graphics::Color::new(r, g, b, 1.0),
        }
    }

    /// https://www.myphysicslab.com/pendulum/double-pendulum-en.html
    ///
    /// This function implements the two equations under (16)
    fn compute_acceleration(&self) -> (f32, f32) {
        let m1 = self.p1.mass;
        let m2 = self.p2.mass;
        let l1 = self.p1.radius;
        let l2 = self.p2.radius;
        let t1 = self.p1.theta;
        let t2 = self.p2.theta;
        let s1sq = self.p1.speed * self.p1.speed;
        let s2sq = self.p2.speed * self.p2.speed;
        let g = GRAVITY;

        let n1 = -g * (2.0 * m1 + m2) * t1.sin();
        let n2 = -m2 * g * (t1 - 2.0 * t2).sin();
        let n3 = -2.0 * (t1 - t2).sin() * m2;
        let n4 = s2sq * l2 + s1sq * l1 * (t1 - t2).cos();
        let num1 = n1 + n2 + n3 * n4;

        let n1 = 2.0 * (t1 - t2).sin();
        let n2 = s1sq * l1 * (m1 + m2);
        let n3 = g * (m1 + m2) * t1.cos() + s2sq * l2 * m2 * (t1 - t2).cos();
        let n4 = s2sq * l2 * m2 * (t1 - t2).cos();
        let num2 = n1 * (n2 + n3 + n4);

        let denom_cos = (2.0 * (t1 - t2)).cos();
        let denom = 2.0 * m1 + m2 - m2 * denom_cos;

        let a1 = num1 / (l1 * denom);
        let a2 = num2 / (l2 * denom);
        return (a1, a2);
    }

    /// Advance the simulation one step forward
    fn forward(&mut self) {
        let (a1, a2) = self.compute_acceleration();

        self.p1.speed += a1;
        self.p2.speed += a2;
        self.p1.theta += self.p1.speed;
        self.p2.theta += self.p2.speed;

        // ? Might be useful to uncomment if the pendulum spins a million times
        // ? and f32 precision starts to be noticeable
        // self.p1.theta %= PI / 2.0;
        // self.p2.theta %= PI / 2.0;
    }

    pub fn update_trail(&mut self) {
        let x = self.p1.x() + self.p2.x();
        let y = self.p1.y() + self.p2.y();
        let point = Point2::new(x, y);

        // Push the current trail position if it's not the same as the previous one
        if let Some(p) = self.trail.back() {
            // ? Should check if the distance is smaller than a threshold
            if p == &point {
                return;
            }
        }
        self.trail.push_back(point);
        if self.trail.len() > 100 {
            self.trail.pop_front();
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.forward();

        self.update_trail();
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let x_1 = self.p1.x();
        let y_1 = self.p1.y();
        let x_2 = x_1 + self.p2.x();
        let y_2 = y_1 + self.p2.y();

        let origin = Point2::new(0.0, 0.0);
        let p1 = Point2::new(x_1, y_1);
        let p2 = Point2::new(x_2, y_2);

        let line = Mesh::new_line(ctx, &[origin, p1, p2], 2.0, self.color)?;

        let circle_1 = Mesh::new_circle(ctx, DrawMode::fill(), origin, 10.0, 2.0, self.color)?;
        let circle_2 = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            p1,
            4.0 * self.p1.mass,
            2.0,
            self.color,
        )?;
        let circle_3 = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            p2,
            4.0 * self.p2.mass,
            2.0,
            self.color,
        )?;

        let center = Point2::new(CENTER.0, CENTER.1);
        graphics::draw(ctx, &line, (center,))?;
        graphics::draw(ctx, &circle_1, (center,))?;
        graphics::draw(ctx, &circle_2, (center,))?;
        graphics::draw(ctx, &circle_3, (center,))?;

        if self.trail.len() >= 3 {
            let trail = Mesh::new_line(
                ctx,
                self.trail.make_contiguous(),
                2.0,
                [0.1, 0.5, 0.1, 1.0].into(),
            )?;
            graphics::draw(ctx, &trail, (center,))?;
        }

        Ok(())
    }
}
