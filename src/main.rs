#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod body;
mod config;
mod controls;
mod fps;
mod style;
mod world;

use crate::body::Body;
use crate::config::window_conf;
use crate::controls::CameraControls;
use crate::fps::FpsChart;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut chart = FpsChart::new(200);
    let mut world = world::World::new();
    let mut pan = CameraControls::new();
    set_camera(&world.camera);

    world.add_body(Body::new(0., 0., 0., 0., 6., 0.3));
    world.add_body(Body::new(1., 0., 0., 1.8, 0.05, 0.1));

    loop {
        set_camera(&world.camera);

        clear_background(BLACK);
        let dt = 0.008; //get_frame_time();
                        // let start = Instant::now();
        world.update(dt);
        // println!("physics: {:?}", start.elapsed());
        // let start = Instant::now();
        world.move_and_draw(dt);
        // println!("draw: {:?}", start.elapsed());
        pan.draw_add();
        set_default_camera();
        draw_fps();
        if !world.draw_ui() {
            if !Rect::new(0., 100., 200., 60.).contains(Vec2::from(mouse_position())) {
                pan.handle_zoom(&mut world.camera);
                pan.handle_pan(&mut world.camera);
                pan.handle_add(&mut world);
            }
        } else {
            pan.cancel_all();
        }
        pan.ar = world.settings.n_radius.parse::<f32>().unwrap_or(pan.ar);
        pan.am = world.settings.n_mass.parse::<f32>().unwrap_or(pan.am);
        chart.record(get_frame_time());
        chart.draw((screen_width() - 100., 5., 100., 50.0), 120.0, 1., GREEN);
        next_frame().await
    }
}
