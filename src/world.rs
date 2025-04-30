use crate::body::{gravity, Body};
use macroquad::prelude::*;
use macroquad::ui::root_ui;

struct Settings {
    pub collisions: bool,
    pub tidal: bool,
    pub follow: bool,
}

pub struct World {
    pub bodies: Vec<Body>,
    pub camera: Camera2D,
    draws: i32,
    settings: Settings,
}

impl World {
    pub fn new() -> Self {
        let bodies = Vec::new();
        let camera = Camera2D::default();
        World {
            bodies,
            camera,
            draws: 0,
            settings: Settings {
                collisions: true,
                tidal: true,
                follow: false,
            },
        }
    }

    pub fn draw_ui(&mut self) {
        root_ui().label(vec2(0., 20.), &format!("draws: {}", self.draws));
        if root_ui().button(
            vec2(0., 40.),
            format!("collision={}", self.settings.collisions),
        ) {
            self.settings.collisions = !self.settings.collisions;
        }
        if root_ui().button(vec2(0., 60.), format!("tidal={}", self.settings.tidal)) {
            self.settings.tidal = !self.settings.tidal;
        }
        if root_ui().button(vec2(0., 80.), format!("follow={}", self.settings.follow)) {
            self.settings.follow = !self.settings.follow;
        }
    }
    pub fn move_and_draw(&mut self, dt: f32) {
        if self.settings.follow {
            let mut center = Vec2::new(0., 0.);
            let mut mass = 0.;
            for body in &self.bodies {
                center += body.p * body.m;
                mass += body.m;
            }
            center /= mass;
            self.camera.target = center;
        }

        let v = self.viewport();
        self.draws = 0;
        for body in &mut self.bodies {
            body.update(dt);
            if body.bb().overlaps(&v) {
                body.draw();
                self.draws += 1;
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        let n = self.bodies.len();
        self.bodies.reserve(self.bodies.len() / 10);
        for i in 0..n {
            for j in (i + 1)..n {
                let mut nb1: Option<Body> = None;
                let mut nb2: Option<Body> = None;
                {
                    let (left, right) = self.bodies.split_at_mut(j);
                    let b1 = &mut left[i];
                    let b2 = &mut right[0];
                    let gravity = gravity(b1, b2);
                    b1.apply_force(gravity, dt);
                    b2.apply_force(-gravity, dt);
                    if self.settings.tidal {
                        let dist = b1.p.distance(b2.p);
                        let roche_b1 = b2.r * (2.0 * b2.rho / b1.rho).powf(1.0 / 3.0);
                        let roche_b2 = b1.r * (2.0 * b1.rho / b2.rho).powf(1.0 / 3.0);
                        let split_b1 = dist < roche_b1;
                        let split_b2 = dist < roche_b2;
                        if split_b1 {
                            nb1 = Some(b1.split_against(b2));
                        }
                        if split_b2 {
                            nb2 = Some(b2.split_against(b1));
                        }
                    }
                }
                if let Some(new_body) = nb1 {
                    self.bodies.push(new_body);
                }
                if let Some(new_body) = nb2 {
                    self.bodies.push(new_body);
                }
            }
        }
        if self.settings.collisions {
            self.handle_collisions();
        }
    }

    fn handle_collisions(&mut self) {
        let mut i = 0;
        while i < self.bodies.len() {
            let mut j = i + 1;
            while j < self.bodies.len() {
                let (left, right) = self.bodies.split_at_mut(j);
                let b1 = &mut left[i];
                let b2 = &mut right[0];
                if b1.intersects(b2) {
                    b1.fuse(b2);
                    self.bodies.swap_remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    fn viewport(&self) -> Rect {
        let tl = self.camera.screen_to_world(Vec2::new(0., 0.));
        let br = self
            .camera
            .screen_to_world(Vec2::new(screen_width(), screen_height()));
        Rect::new(tl.x, tl.y, br.x - tl.x, br.y - tl.y)
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}
