use macroquad::color::Color;
use macroquad::prelude::draw_line;
use std::collections::VecDeque;

pub struct FpsChart {
    samples: VecDeque<(f32, f32)>,
    cap: usize,
}

fn to_screen(
    i: usize,
    (_ft, fps): (f32, f32),
    total: usize,
    bounds: (f32, f32, f32, f32),
    max_fps: f32,
) -> (f32, f32) {
    let (x0, y0, w, h) = bounds;
    (
        x0 + (i as f32) / ((total - 1).max(1) as f32) * w,
        y0 + (1.0 - (fps / max_fps).clamp(0.0, 1.0)) * h,
    )
}
impl FpsChart {
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(capacity),
            cap: capacity,
        }
    }

    pub fn record(&mut self, frame_time: f32) {
        let fps = 1.0 / frame_time.max(1e-6);
        if self.samples.len() == self.cap {
            self.samples.pop_front();
        }
        self.samples.push_back((frame_time, fps));
    }

    pub fn draw(
        &self,
        bounds: (f32, f32, f32, f32),
        max_fps: f32,
        line_thickness: f32,
        col: Color,
    ) {
        let total = self.samples.len();
        if total < 2 {
            return;
        }
        let pts: Vec<(f32, f32)> = self
            .samples
            .iter()
            .enumerate()
            .map(|(i, &data)| to_screen(i, data, total, bounds, max_fps))
            .collect();
        for win in pts.windows(2) {
            let (x0, y0) = win[0];
            let (x1, y1) = win[1];
            draw_line(x0, y0, x1, y1, line_thickness, col);
        }
    }
}
