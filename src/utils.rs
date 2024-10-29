pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}
