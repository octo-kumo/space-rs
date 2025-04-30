use crate::body::Body;
use crate::world::World;
use macroquad::prelude::*;

pub struct CameraControls {
    d: bool,  // dragging
    sp: Vec2, // start position
    tz: f32,  // target zoom
    z: f32,   // current zoom
    adding: bool,
    add_start: Vec2, // start position (world)
    add_end: Vec2,   // end position (world)
}

impl CameraControls {
    pub fn new() -> Self {
        Self {
            d: false,
            sp: Vec2::ZERO,
            tz: 1.0,
            z: 1.0,
            adding: false,
            add_start: Vec2::ZERO,
            add_end: Vec2::ZERO,
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
        if is_mouse_button_released(MouseButton::Middle) {
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
            self.adding = true;
            self.add_start = world.camera.screen_to_world(Vec2::from(mouse_position()));
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.add_end = world.camera.screen_to_world(Vec2::from(mouse_position()));
            self.adding = false;
            let v = self.add_end - self.add_start;
            let body = Body::new(self.add_start.x, self.add_start.y, v.x, v.y, 0.1, 0.1);
            world.add_body(body);
        }
        if self.adding && is_mouse_button_down(MouseButton::Left) {
            self.add_end = world.camera.screen_to_world(Vec2::from(mouse_position()));
        }
    }
}
