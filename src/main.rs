#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod body;
mod config;
mod controls;
mod world;

use crate::body::Body;
use crate::config::window_conf;
use crate::controls::CameraControls;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = world::World::new();
    let mut pan = CameraControls::new();
    set_camera(&world.camera);

    world.add_body(Body::new(0., 0., 0., 0., 6., 0.3));
    world.add_body(Body::new(1., 0., 0., 1.8, 0.05, 0.1));

    loop {
        pan.handle_zoom(&mut world.camera);
        pan.handle_pan(&mut world.camera);
        pan.handle_add(&mut world);
        set_camera(&world.camera);

        clear_background(BLACK);
        let dt = 0.008; //get_frame_time();
                        // let start = Instant::now();
        world.update(dt);
        // println!("physics: {:?}", start.elapsed());
        // let start = Instant::now();
        world.move_and_draw(dt);
        // println!("draw: {:?}", start.elapsed());

        set_default_camera();
        draw_fps();
        world.draw_ui();
        next_frame().await
    }
}
