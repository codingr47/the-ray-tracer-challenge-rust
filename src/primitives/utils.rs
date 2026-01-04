use crate::primitives::coordinates::Coordinates;
use crate::primitives::moveable::Moveable;
use crate::primitives::point::Point;
use crate::primitives::vector::Vector;

pub fn GetTuple(x: f32, y: f32, z: f32, w: i32)  -> Box<dyn Moveable> {
    if w == 0 {
        return Box::new(Vector(Coordinates(x, y, z)))
    } else {
        return Box::new(Point(Coordinates(x, y, z)))
    }
}