#[macro_export]
macro_rules! impl_tuple_add_scalar {
    ($type:ty) => {
        impl Add<f32> for $type {
            
            type Output = Self;

            fn add(self, scalar: f32) -> Self {
                return Self(
                    Coordinates(
                        self.0.X() + scalar,
                        self.0.Y() + scalar,
                        self.0.Z() + scalar,
                ));
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tuple_mul_scalar {
    ($type:ty) => {
        impl Mul<f32> for $type {
            type Output = Self;

            fn mul(self, scalar: f32) -> Self::Output {
                Self(
                    Coordinates(scalar * self.0.X(), scalar * self.0.Y(), scalar * self.0.Z())
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tuple_eq {
    ($type:ty) => {
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                return 
                equal( self.0.X(), other.0.X()) 
                && equal(self.0.Y(), other.0.Y())
                && equal(self.0.Z(), other.0.Z())
                    
            }

            fn ne(&self, other: &Self) -> bool {
                !self.eq(other)
            }
        }
    };
}