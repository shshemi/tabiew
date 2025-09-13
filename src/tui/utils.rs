#[derive(Debug, Default, Clone)]
pub struct Scroll {
    val: usize,
    max: usize,
}

impl Scroll {
    pub fn up(&mut self) {
        self.val = self.val.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.val = self.val.saturating_add(1).min(self.max);
    }

    pub fn adjust(&mut self, lines: usize, height: u16) {
        self.max = lines.saturating_sub(height.into());
        self.val = self.val.min(self.max);
    }

    pub fn val_u16(&self) -> u16 {
        self.val as u16
    }
}
