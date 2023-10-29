use crate::Backend;

pub mod ellipse;
pub mod line;

pub trait Draw<B: Backend> {
    fn draw(&self, target: &mut B::Target);
}
