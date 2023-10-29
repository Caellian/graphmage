use femtovg::{Color, Paint, Path};
use glam::Vec2;

use crate::Backend;

use super::*;
use crate::*;

#[derive(Debug, Default)]
pub enum LineStyle {
    #[default]
    Straight,
}

#[derive(Debug, Default)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub style: LineStyle,
}

impl Draw<BackendFemtovg> for Line {
    fn draw(&self, target: &mut <BackendFemtovg as Backend>::Target) {
        target.save();

        let mut path = Path::new();
        path.move_to(self.start.x, self.start.y);
        path.line_to(self.end.x, self.end.y);

        let paint = Paint::color(Color::rgb(0, 0, 0)).with_line_width(2.0);

        target.stroke_path(&path, &paint);

        target.restore();
    }
}
