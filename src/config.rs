use macroquad::prelude::*;
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Space 2D".to_owned(),
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}
