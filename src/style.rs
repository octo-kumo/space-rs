use macroquad::color::Color;
use macroquad::ui::{root_ui, Skin};
#[inline(always)]
pub fn get_style() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(255, 255, 255, 255))
        .font_size(12)
        .build();
    Skin {
        label_style,
        ..root_ui().default_skin()
    }
}
