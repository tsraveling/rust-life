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

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.data[row * self.width + col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        self.data[row * self.width + col] = val;
    }

    pub fn neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        // iterate from -1 as an i2, to 1. =1 means also include 1 itself.
        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0
                    && r < self.height as i32
                    && c >= 0
                    && c < self.width as i32
                    && self.get(r as usize, c as usize)
                {
                    count += 1;
                }
            }
        }
        count
    }
}
