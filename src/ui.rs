use cursive::{View, Printer, Vec2};
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

    pub fn update(&mut self, incoming: Array2<bool>) {
        self.side = incoming;
    }
}

impl View for SideView {
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(self.side.dim().0, self.side.dim().1)
    }

    fn draw(&self, printer: &Printer) {
        for (pos, value) in self.side().indexed_iter() {
            let ch = if *value { "█" } else { "." };
            printer.print((pos.1, pos.0), &ch);
        }
    }
}