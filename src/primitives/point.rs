use crate::primitives::{coordinates::Coordinates, moveable::Moveable, vector::Vector};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Point(pub Coordinates);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self(Coordinates(x, y, z)) }

    pub fn X(&self) -> f32 { self.0.X() }
    pub fn Y(&self) -> f32 { self.0.Y() }
    pub fn Z(&self) -> f32 { self.0.Z() }
}

impl Moveable for Point {
     fn get_type(&self) -> &'static str {
        return "Point";
     }
}

impl Add for Point {
    type Output = Point;

     fn add(self, other: Self) -> Self {
        return Self(
            Coordinates(
                self.X() + other.X(),
                self.Y() + other.Y(),
                self.Z() + other.Z(),
        ));
    }
}

impl Add<Vector> for Point {
    type Output = Vector;

     fn add(self, other: Vector) -> Vector {
          return Vector(
            Coordinates(
                self.X() + other.X(),
                self.Y() + other.Y(),
                self.Z() + other.Z(),
        ));
    }
}

impl Sub for Point {
    type Output = Vector;

     fn sub(self, other: Self) -> Vector {
        return Vector(
            Coordinates(
                self.X() - other.X(),
                self.Y() - other.Y(),
                self.Z() - other.Z(),
        ));
    }
}

impl Sub<Vector> for Point {
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

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
            Point(
                Coordinates(-self.X(), -self.Y(), -self.Z())
            )
    }
}

//By Scalar
impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Self::Output {
        Point(
            Coordinates(scalar * self.X(), scalar * self.Y(), scalar * self.Z())
        )
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, divider: f32) -> Self::Output {
        Point (
            Coordinates(self.X() / divider, self.Y() / divider, self.Z() / divider)
        )
    }
}