use std::ops::{Add, Sub, Mul};
use crate::utils::math::equal;

use crate::{impl_tuple_add_scalar, impl_tuple_eq, impl_tuple_mul_scalar, primitives::{coordinates::Coordinates, moveable::Moveable}};

#[derive(Debug, Clone)]
pub struct Color (pub Coordinates);

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self { Self(Coordinates(red, green, blue)) }

    pub fn red(&self) -> f32 { self.0.X() }
    pub fn green(&self) -> f32 { self.0.Y() }
    pub fn blue(&self) -> f32 { self.0.Z() }
}

impl Moveable for Color {
     fn get_type(&self) -> &'static str {
        return "Color";
     }
}

impl_tuple_add_scalar!(Color);

impl Add<Color> for Color {
    
    type Output = Self;

    fn add(self, other: Color) -> Self::Output {
        Self::new(
            self.red() + other.red(),
            self.green() + other.green(),
            self.blue() + other.blue()
        )
    }
}

impl Sub<Color> for Color {
    
    type Output = Self;

    fn sub(self, other: Color) -> Self::Output {
        Self::new(
            self.red() - other.red(),
            self.green() - other.green(),
            self.blue() - other.blue()
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    
    fn mul(self, other: Color) -> Self::Output {
        Self::new(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue()
        )
    } 
}


//Color multiply By Scalar
impl_tuple_mul_scalar!(Color);

impl_tuple_eq!(Color);