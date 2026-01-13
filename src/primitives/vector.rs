use std::{ops::{Add, Div, Mul, Neg, Sub}};

use crate::{impl_tuple_add_scalar, impl_tuple_eq, impl_tuple_mul_scalar, primitives::{coordinates::Coordinates, moveable::Moveable, point::Point}, utils::math::equal};

#[derive(Debug)]
pub struct Vector (pub Coordinates);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self(Coordinates(x, y, z)) }

    pub fn X(&self) -> f32 { self.0.X() }
    pub fn Y(&self) -> f32 { self.0.Y() }
    pub fn Z(&self) -> f32 { self.0.Z() }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(
            f32::powf(self.X(), 2.0) 
            + f32::powf(self.Y(), 2.0) 
            + f32::powf(self.Z(), 2.0)
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        return Self(
            Coordinates(
                self.X() / mag, 
                self.Y() / mag,
                self.Z() / mag
            )
        );
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.X() * other.X() 
        + self.Y() * other.Y()
        + self.Z() * other.Z()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            Coordinates(
                self.Y() * other.Z() - self.Z() * other.Y(),
                self.Z() * other.X() - self.X() * other.Z(),
                self.X() * other.Y() - self.Y() * other.X()    
            )
        )
    }
}

impl Moveable for Vector {
     fn get_type(&self) -> &'static str {
        return "Vector";
     }
}


impl_tuple_add_scalar!(Vector);

impl Add<Vector> for Vector {
    
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self::new(
            self.X() + other.X(),
            self.Y() + other.Y(),
            self.Z() + other.Z()
        )
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


//Vector multiply By Scalar
impl_tuple_mul_scalar!(Vector);


impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, divider: f32) -> Self::Output {
        Vector (
            Coordinates(self.X() / divider, self.Y() / divider, self.Z() / divider)
        )
    }
}

impl_tuple_eq!(Vector);

