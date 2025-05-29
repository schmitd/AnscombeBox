use cursive::{View, Printer};
use ndarray::Array2;

pub struct SideView {
    side: Array2<bool>,
}

impl SideView {
    pub fn new(side: Array2<bool>) -> Self {
        Self {
            side,
        }
    }

    pub fn side(&self) -> &Array2<bool> {
        &self.side
    }
}

impl View for SideView {
    fn draw(&self, printer: &Printer) {
        for (pos, value) in self.side.indexed_iter() {
            let ch = if *value { '█' } else { ' ' };
            printer.print((pos.0, pos.1), &ch.to_string());
        }
    }
}