use macroquad::prelude::*;
#[derive(Debug, Clone)]
pub struct Body {
    pub p: Vec2,
    pub v: Vec2,
    pub m: f32,
    pub r: f32,
    pub rho: f32,
}

impl Body {
    pub fn draw(&self) {
        draw_circle(self.p.x, self.p.y, self.r, WHITE);
    }

    pub fn new(x: f32, y: f32, vx: f32, vy: f32, mass: f32, radius: f32) -> Self {
        Body {
            p: Vec2::new(x, y),
            v: Vec2::new(vx, vy),
            m: mass,
            r: radius,
            rho: density(mass, radius),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.p += self.v * dt;
    }

    pub fn apply_force(&mut self, f: Vec2, dt: f32) {
        self.v += f / self.m * dt;
    }

    pub fn split_against(&mut self, other: &Body) -> Body {
        let r = self.r * 2.1f32.powf(-1.0 / 3.0);
        let dir = (self.p - other.p).normalize_or_zero() * r;

        let cp = self.p - dir;
        let fp = self.p + dir;

        self.p = cp;
        self.m = self.m * 0.5;
        self.r = r;
        self.rho = density(self.m, r);

        Body {
            p: fp,
            v: self.v,
            m: self.m,
            r,
            rho: self.rho,
        }
    }

    pub fn fuse(&mut self, other: &mut Body) {
        let total_mass = self.m + other.m;
        self.p = (self.p * self.m + other.p * other.m) / total_mass;
        self.v = (self.v * self.m + other.v * other.m) / total_mass;
        let volume = self.r.powi(3) + other.r.powi(3);
        self.r = volume.powf(1.0 / 3.0);
        self.m = total_mass;
        self.rho = density(self.m, self.r);
    }

    pub fn intersects(&self, other: &Body) -> bool {
        let d = self.p - other.p;
        // d.length_squared() < self.r.max(other.r).powi(2)
        d.length_squared() < (self.r + other.r).powi(2)
    }

    pub fn bb(&self) -> Rect {
        Rect::new(
            self.p.x - self.r,
            self.p.y - self.r,
            self.r * 2.0,
            self.r * 2.0,
        )
    }
}

pub fn gravity(b1: &Body, b2: &Body) -> Vec2 {
    let d = b2.p - b1.p;
    let d2 = d.length_squared();
    let f = b1.m * b2.m / d2;
    f * d / d2.sqrt()
}

pub fn density(m: f32, r: f32) -> f32 {
    m / (4.0 / 3.0 * std::f32::consts::PI * r.powi(3))
}
