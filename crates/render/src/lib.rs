use context::Context;
use femtovg::{Color, Paint};
use graphmage_graph::{
    property::{FontSizeProperty, LabelProperty},
    Graph,
};
use shapes::{ellipse::Ellipse, line::Line, Draw};

pub mod context;
pub mod shapes;

pub trait Backend {
    type Target;
}

pub struct BackendFemtovg;
impl Backend for BackendFemtovg {
    type Target = femtovg::Canvas<femtovg::renderer::OpenGl>;
}

pub fn draw(ctx: &mut Context, graph: &Graph) {
    let size = ctx.size();

    let default_font_size = 12.0;

    let mut canvas = ctx.canvas();

    canvas.clear_rect(0, 0, size.0, size.1, Color::white());

    for e in graph.edges() {
        let start = match graph.get_node(e.start.node) {
            Some(it) => it.position,
            None => continue,
        };
        let end = match graph.get_node(e.end.node) {
            Some(it) => it.position,
            None => continue,
        };
        Line {
            start,
            end,
            style: shapes::line::LineStyle::Straight,
        }
        .draw(&mut canvas)
    }

    for v in graph.nodes() {
        let text_paint = {
            let size = match v.properties.get_font_size() {
                Some(FontSizeProperty::F32(it)) => *it,
                _ => default_font_size,
            };

            Paint::color(Color::black())
                .with_font_size(size)
                .with_text_align(femtovg::Align::Center)
                .with_text_baseline(femtovg::Baseline::Middle)
        };

        let label = match v.properties.get_label() {
            Some(LabelProperty::String(label)) => Some(label.clone()),
            None => None,
        };

        let (width, height) = if let Some(label) = &label {
            let measure = canvas
                .measure_text(0.0, 0.0, label, &text_paint)
                .expect("unable to measure text");

            (
                measure.glyphs.iter().map(|it| it.width).sum(),
                measure
                    .glyphs
                    .into_iter()
                    .map(|it| it.height.ceil() as u32)
                    .max()
                    .unwrap_or_default() as f32,
            )
        } else {
            (0.0, 0.0)
        };

        Ellipse {
            pos: v.position,
            rx: default_font_size.max(width),
            ry: default_font_size.max(height),
        }
        .draw(&mut canvas);

        if let Some(label) = label {
            canvas
                .fill_text(v.position.x, v.position.y, label, &text_paint)
                .expect("unable to draw text");
        }
    }

    canvas.flush()
}

/*
NOTE: Some of this stuff should be available during layouting phase and
render and layout sub-crates should not be dependant of eachother.

Namely, node growing should be performed as the first stage so layouting can
know what to do.
*/