use crate::body::Body;
use crate::world::World;
use macroquad::prelude::*;

pub struct CameraControls {
    d: bool,     // dragging
    sp: Vec2,    // start position
    tz: f32,     // target zoom
    z: f32,      // current zoom
    a: bool,     // adding
    sa: Vec2,    // start position (world)
    ea: Vec2,    // end position (world)
    pub am: f32, // mass
    pub ar: f32, // radius
}

impl CameraControls {
    pub fn new() -> Self {
        Self {
            d: false,
            sp: Vec2::ZERO,
            tz: 1.0,
            z: 1.0,
            a: false,
            sa: Vec2::ZERO,
            ea: Vec2::ZERO,
            am: 0.1,
            ar: 0.1,
        }
    }

    pub fn handle_zoom(&mut self, cam: &mut Camera2D) {
        let (_x, wy) = mouse_wheel();
        if wy.abs() > 0.0 {
            let y = (wy / 10.0).clamp(-2., 2.);
            self.tz *= 1.1f32.powf(y);
        }

        let dt = get_frame_time();
        let t = 1.0 - (-10. * dt).exp();
        self.z *= (self.tz / self.z).powf(t);

        let (w, h) = (screen_width(), screen_height());
        cam.zoom.x = self.z * 100.0 / w;
        cam.zoom.y = self.z * 100.0 / h;
    }

    pub fn handle_pan(&mut self, cam: &mut Camera2D) {
        if is_mouse_button_pressed(MouseButton::Middle) {
            self.d = true;
            self.sp = Vec2::from(mouse_position());
        }
        if self.d && is_mouse_button_released(MouseButton::Middle) {
            self.d = false;
        }
        if self.d && is_mouse_button_down(MouseButton::Middle) {
            let p = Vec2::from(mouse_position());
            cam.target = cam.screen_to_world(cam.world_to_screen(cam.target) + (self.sp - p));
            self.sp = p;
        }
    }

    pub fn handle_add(&mut self, world: &mut World) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.a = true;
            self.sa = world.camera.screen_to_world(Vec2::from(mouse_position()));
        }
        if self.a && is_mouse_button_pressed(MouseButton::Right) {
            self.a = false;
        }
        if self.a && is_mouse_button_released(MouseButton::Left) {
            self.a = false;
            self.ea = world.camera.screen_to_world(Vec2::from(mouse_position()));
            let v = self.ea - self.sa;
            let body = Body::new(self.sa.x, self.sa.y, v.x, v.y, self.am, self.ar);
            world.add_body(body);
        }
        if self.a && is_mouse_button_down(MouseButton::Left) {
            self.ea = world.camera.screen_to_world(Vec2::from(mouse_position()));
        }
    }

    pub fn draw_add(&self) {
        if self.a {
            let p1 = self.sa;
            let p2 = self.ea;
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.01, WHITE);
            draw_circle_lines(p1.x, p1.y, self.ar, 0.01, WHITE);
        }
    }

    pub fn cancel_all(&mut self) {
        self.a = false;
        self.d = false;
    }
}
