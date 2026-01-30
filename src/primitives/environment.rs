use crate::primitives::vector::Vector;

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector
}

impl Environment {
    pub fn new(g: Vector, w: Vector) -> Self {
        return Environment { gravity: g, wind: w };
    }
}