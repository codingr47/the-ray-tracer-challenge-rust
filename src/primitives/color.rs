use crate::primitives::{coordinates::Coordinates, moveable::Moveable};

#[derive(Debug)]
pub struct Color (pub Coordinates);

impl Color {
    pub fn red(&self) -> f32 { self.0.X() }
    pub fn green(&self) -> f32 { self.0.Y() }
    pub fn blue(&self) -> f32 { self.0.Z() }
}

impl Moveable for Color {
     fn get_type(&self) -> &'static str {
        return "Color";
     }
}