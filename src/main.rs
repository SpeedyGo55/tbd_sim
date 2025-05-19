use lazy_static::lazy_static;
use nannou::prelude::*;

const TIME_STEP: f32 = 0.01;

const SIZE: f32 = 250.0;

lazy_static! {
    static ref BODIES: Vec<Body> = vec![
        Body::new(vec2(0.97000436, -0.24308753)*SIZE, vec2(0.46620368, 0.43236573)*SIZE, 1.0, RED),
        Body::new(vec2(-0.97000436, 0.24308753)*SIZE, vec2(0.46620368, 0.43236573)*SIZE, 1.0, GREEN),
        Body::new(vec2(0.0, 0.0)*SIZE, vec2(-0.93240737, -0.86473146)*SIZE, 1.0, BLUE),
    ];
}
#[derive(Clone, Copy, Debug)]
struct Body {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    mass: f32,
    color: Rgb<u8>,
}

impl Body {
    fn new(pos: Vec2, vel: Vec2, mass: f32, color: Rgb<u8>) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::ZERO,
            mass,
            color,
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acc += force / self.mass;
    }

    fn update(&mut self) {
        self.vel += self.acc * TIME_STEP;
        self.pos += self.vel * TIME_STEP;
        self.acc = Vec2::ZERO;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .w_h(20.0, 20.0)
            .rgb(
                self.color.red as f32 / 255.0,
                self.color.green as f32 / 255.0,
                self.color.blue as f32 / 255.0,
            );
    }
}

struct Model {
    bodies: Vec<Body>,
    running: bool,
    space_down: bool,
}

fn model(_app: &App) -> Model {
    Model { bodies: BODIES.clone(), running: true, space_down: false }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if _app.keys.down.contains(&Key::Space) && !model.space_down {
        model.running = !model.running;
        model.space_down = true;
    } else if !_app.keys.down.contains(&Key::Space) {
        model.space_down = false;
    }

    // on keypress 'r' reset the bodies
    if _app.keys.down.contains(&Key::R) {
        model.bodies = BODIES.clone();
    }
    // if mouse is pressed and on a body drag it
    if _app.mouse.buttons.left().is_down() {
        for body in &mut model.bodies {
            if body.pos.distance(_app.mouse.position()) < 30.0 {
                body.pos = _app.mouse.position();
                body.vel = Vec2::ZERO;
            }
        }
    }
    if model.running {
        let G: f32 = 1.0 * SIZE.powi(3); // Gravitational constant
        let mut forces = vec![vec2(0.0, 0.0); model.bodies.len()];

        for i in 0..model.bodies.len() {
            for j in 0..model.bodies.len() {
                if i != j {
                    let dir = model.bodies[j].pos - model.bodies[i].pos;
                    let dist_sq = dir.length_squared().max(5.0);
                    let force_mag = G * model.bodies[i].mass * model.bodies[j].mass / dist_sq;
                    let force = dir.normalize() * force_mag;
                    forces[i] += force;
                }
            }
        }

        for (body, force) in model.bodies.iter_mut().zip(forces.iter()) {
            body.apply_force(*force);
            body.update();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    } else {
        draw.rect().w_h(8000.0, 8000.0).color(srgba(0.0, 0.0, 0.0, 0.05));
    }

    for body in &model.bodies {
        body.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
