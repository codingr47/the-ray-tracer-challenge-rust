use crate::{impl_tuple_eq, impl_tuple_mul_scalar, primitives::{coordinates::Coordinates, moveable::Moveable, vector::Vector}};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::utils::math::equal;

#[derive(Debug, Clone)]
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

//Point multiply By Scalar
impl_tuple_mul_scalar!(Point);

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, divider: f32) -> Self::Output {
        Point (
            Coordinates(self.X() / divider, self.Y() / divider, self.Z() / divider)
        )
    }
}

impl_tuple_eq!(Point);