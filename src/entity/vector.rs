#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new() -> Self {
        Vector { x: 0f32, y: 0f32 }
    }
}
