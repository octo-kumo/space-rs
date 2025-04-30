use macroquad::prelude::*;
#[derive(Debug, Clone)]
pub struct Body {
    pub p: Vec2,
    pub v: Vec2,
    pub m: f32,
    pub r: f32,
    pub rho: f32,
}
const SPLIT_FAC: f32 = 0.78089666;
impl Body {
    #[inline(always)]
    pub fn draw(&self) {
        draw_circle(self.p.x, self.p.y, self.r, WHITE);
    }
    #[inline(always)]
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, mass: f32, radius: f32) -> Self {
        Body {
            p: Vec2::new(x, y),
            v: Vec2::new(vx, vy),
            m: mass,
            r: radius,
            rho: density(mass, radius),
        }
    }
    #[inline(always)]
    pub fn update(&mut self, dt: f32) {
        // self.v = self.v.clamp_length(0.0, 1000.0);
        self.p += self.v * dt;
        // println!("p={} v={}", self.p, self.v)
    }
    #[inline(always)]
    pub fn apply_force(&mut self, f: Vec2, dt: f32) {
        self.v += f / self.m * dt;
    }
    #[inline(always)]
    pub fn split_against(&mut self, other: &Body) -> Body {
        let disp = self.p - other.p;
        let d2 = disp.length_squared();
        let r_new = self.r * SPLIT_FAC;
        let axis = disp * (1.0 / d2.sqrt());

        let cp = self.p - axis * r_new;
        let fp = self.p + axis * r_new;
        let denom = (d2 - (r_new * r_new)).max(1e-6);
        let dv = (other.m * r_new / denom).sqrt();
        let v_near = self.v - axis * dv;
        let v_far = self.v + axis * dv;
        let hm = self.m * 0.5;
        self.p = cp;
        self.v = v_near;
        self.m -= hm;
        self.r = r_new;
        self.rho = density(self.m, r_new);

        Body {
            p: fp,
            v: v_far,
            m: hm,
            r: r_new,
            rho: self.rho,
        }
    }
    #[inline(always)]
    pub fn fuse(&mut self, other: &mut Body) {
        let total_mass = self.m + other.m;
        self.p = (self.p * self.m + other.p * other.m) / total_mass;
        self.v = (self.v * self.m + other.v * other.m) / total_mass;
        self.r = (self.r.powi(3) + other.r.powi(3)).cbrt();
        self.m = total_mass;
        self.rho = density(self.m, self.r);
    }
    #[inline(always)]
    pub fn intersects(&self, other: &Body) -> bool {
        let d = self.p - other.p;
        // d.length_squared() < self.r.max(other.r).powi(2)
        d.length_squared() < (self.r + other.r).powi(2)
    }
    #[inline(always)]
    pub fn bb(&self) -> Rect {
        Rect::new(
            self.p.x - self.r,
            self.p.y - self.r,
            self.r * 2.0,
            self.r * 2.0,
        )
    }
}
#[inline(always)]
pub fn gravity(b1: &Body, b2: &Body) -> Vec2 {
    let d = b2.p - b1.p;
    let d2 = d.length_squared();
    (b1.m * b2.m / d2) * d / d2.sqrt()
}

const INV_FOUR_THIRDS_PI: f32 = 3.0 / (4.0 * std::f32::consts::PI);

#[inline(always)]
pub fn density(m: f32, r: f32) -> f32 {
    m * INV_FOUR_THIRDS_PI / (r * r * r)
}
