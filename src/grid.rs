const NOISE_AMT: f32 = 0.1f32;

use rand::rng;
use rand::seq::index::sample;

pub struct Grid {
    data: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            data: vec![false; width * height], // macro: make vec at w*h size, all false
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        self.data[y * self.width + x] = val;
    }

    pub fn add_noise(&mut self) {
        let total = self.width * self.height;
        let amt: usize = (total as f32 * NOISE_AMT) as usize;
        let indices = sample(&mut rng(), total, amt);
        for i in indices {
            self.data[i] = true;
        }
    }

    pub fn neighbor_count(&self, ax: usize, ay: usize) -> u8 {
        let mut count = 0;
        // iterate from -1 as an i2, to 1. =1 means also include 1 itself.
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let gx = x + ax as isize;
                let gy = y + ay as isize;
                if gx >= 0
                    && gy > 0
                    && gx < self.width as isize
                    && gy < self.height as isize
                    && self.get(gx as usize, gy as usize)
                {
                    count += 1;
                }
            }
        }
        count
    }
}
