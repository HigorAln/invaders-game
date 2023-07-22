use crate::frame::{Drawable, Frame};

#[derive(Default)]
pub struct Score {
    point: u16,
}

impl Score {
    pub fn new() -> Self {
        Self { point: 0 }
    }

    pub fn add_points(&mut self, amount: u16) {
        self.point += amount;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        let formatted = format!("SCORE: {:0>4}", self.point);

        for (i, c) in formatted.chars().enumerate() {
            frame[i][0] = c;
        }
    }
}
