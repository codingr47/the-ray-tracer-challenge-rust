#[derive(Clone, Copy)]
pub struct Coordinates(pub f32, pub f32, pub f32);

impl Coordinates {
    pub fn X(&self) -> f32 { self.0 }
    pub fn Y(&self) -> f32 { self.1 }
    pub fn Z(&self) -> f32 { self.2 }
} 