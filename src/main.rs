#![windows_subsystem = "windows"]

use lazy_static::lazy_static;
use nannou::prelude::*;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

const TIME_STEP: f32 = 0.01;

const SIZE: f32 = 250.0;

lazy_static! {
    static ref BODIES: Vec<Body> = {
        let path = std::env::current_exe()
            .expect("Unable to get current executable path")
            .parent()
            .expect("Unable to get parent directory")
            .join("config.json");
        let mut bodies = load_bodies_json(Box::new(path));
        for body in &mut bodies {
            body.pos = body.pos * SIZE;
            body.vel = body.vel * SIZE;
        }
        bodies
    };
}

fn load_bodies_json(filepath: Box<PathBuf>) -> Vec<Body> {
    let file = std::fs::File::open(filepath.as_path()).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    let bodies: Vec<Body> = serde_json::from_reader(reader).expect("Unable to parse JSON");
    bodies
}

fn save_bodies_json(filepath: Box<PathBuf>, bodies: &Vec<Body>) {
    if let Some(parent) = filepath.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create directories: {}", e);
            return;
        }
    }
    let file = match std::fs::File::create(filepath.as_path()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to create file: {}", e);
            return;
        }
    };
    let writer = std::io::BufWriter::new(file);
    if let Err(e) = serde_json::to_writer(writer, bodies) {
        eprintln!("Unable to write JSON: {}", e);
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
        let r = (self.mass / f32::PI()).sqrt();
        let r = r * 20.0;
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .w_h(r * 2.0 , r * 2.0)
            .rgb(
                self.color.red as f32 / 255.0,
                self.color.green as f32 / 255.0,
                self.color.blue as f32 / 255.0,
            );
    }
}

struct Model {
    initial_bodies: Vec<Body>,
    bodies: Vec<Body>,
    running: bool,
    space_down: bool,
    selected_body: Option<usize>,
}

fn model(_app: &App) -> Model {
    Model {
        initial_bodies: BODIES.clone(),
        bodies: BODIES.clone(),
        running: true,
        space_down: false,
        selected_body: None,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.keys.down.contains(&Key::Space) && !model.space_down {
        model.running = !model.running;
        model.space_down = true;
    } else if !app.keys.down.contains(&Key::Space) {
        model.space_down = false;
    }

    if app.keys.down.contains(&Key::R) {
        model.bodies = model.initial_bodies.clone();
    }

    if app.mouse.buttons.left().is_down() {
        match model.selected_body {
            Some(index) => {
                let body = &mut model.bodies[index];
                body.pos = app.mouse.position();
                body.vel = Vec2::ZERO;
            }
            None => {
                for (i, body) in model.bodies.iter_mut().enumerate() {
                    let r = (body.mass / f32::PI()).sqrt() * 20.0;
                    if body.pos.distance(app.mouse.position()) < r + 1.0 {
                        model.selected_body = Some(i);
                        break;
                    }
                }
            }
        }
    } else {
        model.selected_body = None;
    }

    if app.keys.down.contains(&Key::S) {
        if let Some(path) = FileDialog::new()
            .set_title("Save Body")
            .add_filter("JSON", &["json"])
            .save_file()
        {
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                        eprintln!("Failed to create directories: {}", e);
                    });
                }
            }
            let mut bodies = model.bodies.clone();
            for body in &mut bodies {
                body.pos = body.pos / SIZE;
                body.vel = body.vel / SIZE;
            }

            save_bodies_json(Box::new(path), &bodies);
        }
    }

    if app.keys.down.contains(&Key::L) {
        if let Some(path) = FileDialog::new()
            .set_title("Load Body")
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            let mut bodies = load_bodies_json(Box::new(path));
            for body in &mut bodies {
                body.pos = body.pos * SIZE;
                body.vel = body.vel * SIZE;
            }
            model.bodies = bodies.clone();
            model.initial_bodies = bodies.clone();
        }
    }

    if model.running {
        let G: f32 = 1.0 * SIZE.powi(3);
        let mut forces = vec![vec2(0.0, 0.0); model.bodies.len()];

        for i in 0..model.bodies.len() {
            for j in (i + 1)..model.bodies.len() {
                let dir = model.bodies[j].pos - model.bodies[i].pos;
                let dist_sq = dir.length_squared().max(5.0);
                let force_mag = G * model.bodies[i].mass * model.bodies[j].mass / dist_sq;
                let force = dir.normalize() * force_mag;
                forces[i] += force;
                forces[j] -= force;
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
    let win = app.window_rect();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .w_h(win.w(), win.h())
            .color(srgba(0.0, 0.0, 0.0, 0.05));
    }

    for body in &model.bodies {
        body.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .fullscreen()
        .run();
}
