use std::time::Duration;

use rusty_time::timer::Timer;

use crate::NUM_COLS;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_COLS / 2 {
                if (x > 1)
                    && (y > 0)
                    && (x < NUM_COLS - 2)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y })
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        false
        // self.move_timer.update(delta);
    }
}
