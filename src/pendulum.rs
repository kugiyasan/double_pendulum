use std::collections::VecDeque;

use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use rand::Rng;
use std::f32::consts::PI;

/// I know gravity is 9.80m/s^2 in real life, but this is a simulation
const GRAVITY: f32 = 1.0;
/// The number of previous positions stored for the trail
const TRAIL_LENGTH: usize = 100;

// Useful resources:
// https://www.myphysicslab.com/pendulum/double-pendulum-en.html
// https://en.wikipedia.org/wiki/Double_pendulum#Lagrangian
// https://en.wikipedia.org/wiki/Euler_method

/// A single pendulum used to store data for its physics calculation
struct Pendulum {
    /// The mass of the circle (the lines have zero mass)
    /// This also affects the size of the circle
    mass: f32,
    /// The length of the rod, in pixels
    radius: f32,
    /// The angle of the pendulum in radians (0 is pointing down, PI/2 is pointing right)
    theta: f32,
    /// The speed at which the pendulum moves
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
    /// The first pendulum connected to the origin
    p1: Pendulum,
    /// The second pendulum attached at the tip of p1
    p2: Pendulum,
    trail: VecDeque<Point2<f32>>,
    color: graphics::Color,
}

impl DoublePendulum {
    /// Create a new DoublePendulum with a random initial state
    ///
    /// The double pendulum will spawn straight in the top half with no initial speed
    pub fn new() -> Self {
        // TODO make the length dependant of the screen size
        let length = 200.0 / 2.0;
        let mut rng = rand::thread_rng();

        let m1 = rng.gen_range(2.0..5.0);
        let m2 = rng.gen_range(2.0..5.0);
        let radius = rng.gen_range(-50.0..50.0);
        let theta = rng.gen_range(0.0..PI) + PI / 2.0;

        let r = rng.gen_range(0.0..=1.0);
        let g = rng.gen_range(0.0..=1.0);
        let b = rng.gen_range(0.0..=1.0);

        Self {
            p1: Pendulum::new(m1, length + radius, theta, 0.0),
            p2: Pendulum::new(m2, length - radius, theta, 0.0),
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
            color: graphics::Color::new(r, g, b, 1.0),
        }
    }

    /// https://www.myphysicslab.com/pendulum/double-pendulum-en.html
    ///
    /// This function implements the two equations under (16)
    ///
    /// The function returns a1 and a2,
    /// which are the angular acceleration of both pendulums
    fn compute_acceleration(&self) -> (f32, f32) {
        // Name the variables in a similar fashion to the website
        let m1 = self.p1.mass;
        let m2 = self.p2.mass;
        let l1 = self.p1.radius;
        let l2 = self.p2.radius;
        let t1 = self.p1.theta;
        let t2 = self.p2.theta;
        let s1sq = self.p1.speed * self.p1.speed;
        let s2sq = self.p2.speed * self.p2.speed;
        let g = GRAVITY;

        // Make the `sin` and `cos` syntax more natural
        let sin = f32::sin;
        let cos = f32::cos;

        // Compute the first numerator
        let n1 = g * (2.0 * m1 + m2) * sin(t1);
        let n2 = m2 * g * sin(t1 - 2.0 * t2);
        let n3 = -2.0 * sin(t1 - t2) * m2;
        let n4 = s2sq * l2 + s1sq * l1 * cos(t1 - t2);
        let num1 = -n1 - n2 - n3 * n4;

        // Compute the second numerator
        let n1 = 2.0 * sin(t1 - t2);
        let n2 = s1sq * l1 * (m1 + m2);
        let n3 = g * (m1 + m2) * cos(t1) + s2sq * l2 * m2 * cos(t1 - t2);
        let n4 = s2sq * l2 * m2 * cos(t1 - t2);
        let num2 = n1 * (n2 + n3 + n4);

        // Compute the denumerator (it is almost the same denominator for both accelerations)
        let denom = 2.0 * m1 + m2 - m2 * cos(2.0 * (t1 - t2));

        let a1 = num1 / (l1 * denom);
        let a2 = num2 / (l2 * denom);
        return (a1, a2);
    }

    /// Advance the simulation one step forward
    fn forward(&mut self) {
        let (a1, a2) = self.compute_acceleration();

        // TODO Should make this code time-dependant instead of step-based
        // TODO Should make sure that we don't start spinning weirdly because of the lack of resistance
        // ? Maybe add a speed limit
        // ? Maybe make sure to keep the same mechanic energy through the whole simulation
        // ! Should make sure that theta and speed is a finite f32, or else ggez will crash
        self.p1.speed += a1;
        self.p2.speed += a2;
        self.p1.theta += self.p1.speed;
        self.p2.theta += self.p2.speed;

        // ? Might be useful to uncomment if the pendulum spins a million times
        // ? and f32 precision starts to be noticeable
        // self.p1.theta %= PI / 2.0;
        // self.p2.theta %= PI / 2.0;
    }

    /// Update self.trail by popping the oldest point and pushing a new point in it
    fn update_trail(&mut self) {
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
        if self.trail.len() >= TRAIL_LENGTH {
            self.trail.pop_front();
        }
        self.trail.push_back(point);
    }

    fn draw_trail(&mut self, ctx: &mut Context, center: Point2<f32>) -> GameResult {
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

    /// Update the double pendulum and its trail one step forward
    pub fn update(&mut self) -> GameResult {
        self.forward();

        self.update_trail();
        Ok(())
    }

    /// Draw the two lines, the two circles and the trail if it needs to be drawn
    pub fn draw(&mut self, ctx: &mut Context, center: Point2<f32>, show_trail: bool) -> GameResult {
        let x_1 = self.p1.x();
        let y_1 = self.p1.y();
        let x_2 = x_1 + self.p2.x();
        let y_2 = y_1 + self.p2.y();

        let origin = Point2::new(0.0, 0.0);
        let p1 = Point2::new(x_1, y_1);
        let p2 = Point2::new(x_2, y_2);

        // The two lines can be drawn at once
        let line = Mesh::new_line(ctx, &[origin, p1, p2], 2.0, self.color)?;

        let circle_1 = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            p1,
            4.0 * self.p1.mass,
            2.0,
            self.color,
        )?;
        let circle_2 = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            p2,
            4.0 * self.p2.mass,
            2.0,
            self.color,
        )?;

        graphics::draw(ctx, &line, (center,))?;
        graphics::draw(ctx, &circle_1, (center,))?;
        graphics::draw(ctx, &circle_2, (center,))?;

        if show_trail {
            self.draw_trail(ctx, center)?;
        }

        Ok(())
    }
}
