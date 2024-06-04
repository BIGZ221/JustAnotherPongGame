#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn add(&mut self, rhs: &Vec2) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        *self
    }

    pub fn rotate(&mut self, rads: f32) -> Self {
        let old_x = self.x.to_owned();
        let old_y = self.y.to_owned();
        let sin = rads.sin();
        let cos = rads.cos();
        self.x = cos * old_x - sin * old_y;
        self.y = sin * old_x + cos * old_y;
        *self
    }
}
