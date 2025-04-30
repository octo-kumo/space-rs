use crate::body::{gravity, Body};
use crate::style::get_style;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::InputText;

pub struct Settings {
    pub collisions: bool,
    pub tidal: bool,
    pub follow: bool,
    pub n_mass: String,
    pub n_radius: String,
    pub n_cap: String,
    _n_cap: usize,
}
pub struct Statistics {
    pub momentum: Vec2,
    pub mass: f32,
    pub center: Vec2,
}

pub struct World {
    pub bodies: Vec<Body>,
    pub camera: Camera2D,
    draws: i32,
    pub settings: Settings,
    pub stats: Statistics,
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
                n_mass: String::from("0.1"),
                n_radius: String::from("0.1"),
                n_cap: String::from("1000"),
                _n_cap: 1000,
            },
            stats: Statistics {
                momentum: vec2(0., 0.),
                mass: 0.0,
                center: vec2(0., 0.),
            },
        }
    }

    pub fn draw_ui(&mut self) -> bool {
        let skin = get_style();
        root_ui().push_skin(&skin);
        let mut handled = false;
        root_ui().label(
            vec2(0., 20.),
            &format!("draws={} n={}", self.draws, self.bodies.len()),
        );
        root_ui().label(vec2(100., 20.), &format!("mass={}", self.stats.mass));
        root_ui().label(
            vec2(0., 30.),
            &format!(
                "p=({}, {})",
                fmt_coord(self.stats.momentum.x),
                fmt_coord(self.stats.momentum.y)
            ),
        );
        if root_ui().button(
            vec2(0., 40.),
            format!("collision={}", self.settings.collisions),
        ) {
            self.settings.collisions = !self.settings.collisions;
            handled = true;
        }
        if root_ui().button(vec2(0., 60.), format!("tidal={}", self.settings.tidal)) {
            self.settings.tidal = !self.settings.tidal;
            handled = true;
        }
        if root_ui().button(vec2(0., 80.), format!("follow={}", self.settings.follow)) {
            self.settings.follow = !self.settings.follow;
            handled = true;
        }
        if root_ui().button(vec2(0., 100.), "clear bodies") {
            self.bodies.clear();
            handled = true;
        }
        InputText::new(hash!())
            .filter_numbers()
            .label("mass")
            .position(vec2(0., 120.))
            .size(vec2(200., 20.))
            .ui(&mut root_ui(), &mut self.settings.n_mass);
        InputText::new(hash!())
            .filter_numbers()
            .label("radius")
            .position(vec2(0., 120.))
            .size(vec2(200., 20.))
            .ui(&mut root_ui(), &mut self.settings.n_radius);
        InputText::new(hash!())
            .filter_numbers()
            .label("soft n-cap")
            .position(vec2(0., 120.))
            .size(vec2(200., 20.))
            .ui(&mut root_ui(), &mut self.settings.n_cap);

        root_ui().label(vec2(5., screen_height() - 15.), "F1 to toggle UI");
        root_ui().pop_skin();
        handled
    }
    pub fn move_and_draw(&mut self, dt: f32) {
        let mut center = Vec2::new(0., 0.);
        let mut momentum = Vec2::new(0., 0.);
        let mut mass = 0.;
        for body in &self.bodies {
            center += body.p * body.m;
            momentum += body.v * body.m;
            mass += body.m;
        }
        if mass != 0. {
            center /= mass;
        }
        if self.settings.follow {
            self.camera.target = center;
        }
        self.stats.momentum = momentum;
        self.stats.mass = mass;
        self.stats.center = center;

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
        let mut new_bodies = Vec::with_capacity(n / 10);
        let ptr = self.bodies.as_mut_ptr();
        let mut rng = fastrand::Rng::new();
        self.settings._n_cap = self
            .settings
            .n_cap
            .parse::<usize>()
            .unwrap_or(self.settings._n_cap);
        let n_cap = self.settings._n_cap;
        for i in 0..n {
            for j in (i + 1)..n {
                unsafe {
                    let b1 = &mut *ptr.add(i);
                    let b2 = &mut *ptr.add(j);
                    let gravity = gravity(b1, b2);
                    b1.apply_force(gravity, dt);
                    b2.apply_force(-gravity, dt);
                    if self.settings.tidal && n < n_cap && rng.u32(0..10) < 1 {
                        let dist = b1.p.distance(b2.p);
                        let roche_b1 = b2.r * (2.0 * b2.rho / b1.rho).cbrt();
                        let roche_b2 = b1.r * (2.0 * b1.rho / b2.rho).cbrt();
                        let split_b1 = dist < roche_b1;
                        let split_b2 = dist < roche_b2;
                        if split_b1 && b1.r > 0.005 {
                            new_bodies.push(b1.split_against(b2));
                        }
                        if split_b2 && b2.r > 0.005 {
                            new_bodies.push(b2.split_against(b1));
                        }
                    }
                }
            }
        }
        self.bodies.extend(new_bodies);
        if self.settings.collisions {
            self.handle_collisions();
        }
    }
    #[inline(always)]
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

    #[inline(always)]
    fn viewport(&self) -> Rect {
        let tl = self.camera.screen_to_world(Vec2::new(0., 0.));
        let br = self
            .camera
            .screen_to_world(Vec2::new(screen_width(), screen_height()));
        Rect::new(tl.x, tl.y, br.x - tl.x, br.y - tl.y)
    }
    #[inline(always)]
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}

const TOTAL: usize = 8;
#[inline(always)]
fn fmt_coord(v: f32) -> String {
    let int_len = format!("{}", v.trunc() as i32).len();
    format!(
        "{:>width$.prec$}",
        v,
        width = TOTAL,
        prec = if TOTAL > int_len + 1 {
            TOTAL - int_len - 1
        } else {
            0
        }
    )
}
