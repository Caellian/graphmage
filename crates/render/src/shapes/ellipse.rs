use femtovg::{Color, Paint, Path};
use glam::Vec2;

use super::*;
use crate::*;

#[derive(Debug, Default)]
pub struct Ellipse {
    pub pos: Vec2,
    pub rx: f32,
    pub ry: f32,
}

impl Draw<BackendFemtovg> for Ellipse {
    fn draw(&self, target: &mut <BackendFemtovg as Backend>::Target) {
        target.save();

        let mut path = Path::new();
        path.ellipse(self.pos.x, self.pos.y, self.rx, self.ry);

        let paint = Paint::color(Color::rgb(0, 0, 0)).with_line_width(2.0);

        target.stroke_path(&path, &paint);

        target.restore();
    }
}
