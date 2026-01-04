use std::ops::{Add, Mul, Neg, Sub};

use crate::primitives::{coordinates::Coordinates, moveable::Moveable, point::Point};

pub struct Vector (pub Coordinates);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self(Coordinates(x, y, z)) }

    pub fn X(&self) -> f32 { self.0.X() }
    pub fn Y(&self) -> f32 { self.0.Y() }
    pub fn Z(&self) -> f32 { self.0.Z() }
}

impl Moveable for Vector {
     fn get_type(&self) -> &'static str {
        return "Vector";
     }
}


impl Add for Vector {
    type Output = Vector;

     fn add(self, other: Self) -> Self {
         return Self(
            Coordinates(
                self.X() + other.X(),
                self.Y() + other.Y(),
                self.Z() + other.Z(),
        ));
    }
}

impl Add<Point> for Vector {
    type Output = Vector;

     fn add(self, other: Point) -> Vector {
         return Self(
            Coordinates(
                self.X() + other.X(),
                self.Y() + other.Y(),
                self.Z() + other.Z(),
        ));
    }
}

impl Sub for Vector {
    
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        return Point(
            Coordinates(
                self.X() - other.X(),
                self.Y() - other.Y(),
                self.Z() - other.Z(),
        ));
    }

}


impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
            Vector(
                Coordinates(-self.X(), -self.Y(), -self.Z())
            )
    }
}


//By Scalar
impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector(
            Coordinates(scalar * self.X(), scalar * self.Y(), scalar * self.Z())
        )
    }
}
