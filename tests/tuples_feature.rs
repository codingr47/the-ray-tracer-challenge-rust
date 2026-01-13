use rstest::rstest;
use the_raytracer_challenge_rust::primitives::{color::Color, moveable::Moveable, point::Point, utils::*, vector::Vector};

#[rstest]
fn when_w_is_1_A_is_a_point() {
    let A = GetTuple(4.3, -4.2, 3.1, 1);
    assert_eq!(A.get_type(), "Point");
}
#[rstest]
fn when_w_is_0_A_is_a_point() {
    let A = GetTuple(4.3, -4.2, 3.1, 0);
    assert_eq!(A.get_type(), "Vector");
}


#[rstest]
fn when_adding_two_points_you_get_point() {
    let A = Point::new(3.0, -2.0, 3.0);
    let B = Point::new(-2.0, 3.0, 1.0);
    let C = A + B;

    assert_eq!(C.get_type(), "Point");
    assert_eq!(C.X(), 1.0);
    assert_eq!(C.Y(), 1.0);
    assert_eq!(C.Z(), 4.0);
}


#[rstest]
fn when_adding_a_point_to_a_vector_you_get_vector() {
    let A = Point::new(3.0, -2.0, 3.0);
    let V1 = Vector::new(-2.0, 3.0, 1.0);
    let V2 = A + V1;

    assert_eq!(V2.get_type(), "Vector");
    assert_eq!(V2.X(), 1.0);
    assert_eq!(V2.Y(), 1.0);
    assert_eq!(V2.Z(), 4.0);
}

#[rstest]
fn when_adding_two_vectors_you_get_vector() {
    let V1 = Vector::new(3.0, -2.0, 3.0);
    let V2 = Vector::new(-2.0, 3.0, 1.0);
    let V3 = V1 + V2;

    assert_eq!(V3.get_type(), "Vector");
    assert_eq!(V3.X(), 1.0);
    assert_eq!(V3.Y(), 1.0);
    assert_eq!(V3.Z(), 4.0);
}


#[rstest]
fn when_adding_a_vector_to_a_point_you_get_vector() {
    let V = Vector::new(3.0, -2.0, 3.0);
    let A = Point::new(-2.0, 3.0, 1.0);
    let V2 = V + A;

    assert_eq!(V2.get_type(), "Vector");
    assert_eq!(V2.X(), 1.0);
    assert_eq!(V2.Y(), 1.0);
    assert_eq!(V2.Z(), 4.0);
}


#[rstest]
fn when_subtracting_two_points_you_get_vector() {
    let A = Point::new(3.0, 2.0, 1.0);
    let B = Point::new(5.0, 6.0, 7.0);
    let V = A - B;

    assert_eq!(V.get_type(), "Vector");
    assert_eq!(V.X(), -2.0);
    assert_eq!(V.Y(), -4.0);
    assert_eq!(V.Z(), -6.0);
}


#[rstest]
fn when_subtracting_a_vector_from_a_point_you_get_point() {
    let A = Point::new(3.0, 2.0, 1.0);
    let V = Vector::new(5.0, 6.0, 7.0);
    let B = A - V;

    assert_eq!(B.get_type(), "Point");
    assert_eq!(B.X(), -2.0);
    assert_eq!(B.Y(), -4.0);
    assert_eq!(B.Z(), -6.0);
}


#[rstest]
fn when_subtracting_two_vectors_you_get_point() {
    let V1 = Point::new(3.0, 2.0, 1.0);
    let V2 = Vector::new(5.0, 6.0, 7.0);
    let A = V1 - V2;

    assert_eq!(A.get_type(), "Point");
    assert_eq!(A.X(), -2.0);
    assert_eq!(A.Y(), -4.0);
    assert_eq!(A.Z(), -6.0);
}


#[rstest]
fn when_negating_a_point_you_get_negative_values() {
    let A = Point::new(3.0, 2.0, -1.0);
    let negative_A = -A;
    
    assert_eq!(negative_A.get_type(), "Point");
    assert_eq!(negative_A.X(), -3.0);
    assert_eq!(negative_A.Y(), -2.0);
    assert_eq!(negative_A.Z(), 1.0);
}


#[rstest]
fn when_negating_a_vector_you_get_negative_values() {
    let V1 = Vector::new(3.0, 2.0, -1.0);
    let negative_V1 = -V1;
    
    assert_eq!(negative_V1.get_type(), "Vector");
    assert_eq!(negative_V1.X(), -3.0);
    assert_eq!(negative_V1.Y(), -2.0);
    assert_eq!(negative_V1.Z(), 1.0);
}

#[rstest]
fn when_multiplying_a_point_you_get_multiplied_components() {
    let A = Point::new(1.0, -2.0, 3.0);
    let scalar = 3.5;
    let B = A * scalar;

    assert_eq!(B.X(), 3.5);
    assert_eq!(B.Y(), -7.0);
    assert_eq!(B.Z(), 10.5);
}

#[rstest]
fn when_multiplying_a_vector_you_get_multiplied_components() {
    let V1 = Vector::new(1.0, -2.0, 3.0);
    let scalar = 3.5;
    let V2 = V1 * scalar;

    assert_eq!(V2.X(), 3.5);
    assert_eq!(V2.Y(), -7.0);
    assert_eq!(V2.Z(), 10.5);
}


#[rstest]
fn when_dividing_a_point_by_a_float_you_get_divided_components() {
    let A = Point::new(1.0, -2.0, 3.0);
    let divider = 2.0;
    let B = A / divider;

    assert_eq!(B.X(), 0.5);
    assert_eq!(B.Y(), -1.0);
    assert_eq!(B.Z(), 1.5);
}

#[rstest]
fn when_dividing_a_vector_by_a_float_you_get_divided_components() {
    let V1 = Vector::new(1.0, -2.0, 3.0);
    let divider = 2.0;
    let V2 = V1 / divider;

    assert_eq!(V2.X(), 0.5);
    assert_eq!(V2.Y(), -1.0);
    assert_eq!(V2.Z(), 1.5);
}

#[rstest]
fn when_computing_the_magnitodue_of_vector_1_0_0_you_get_1() {
    assert_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
}

#[rstest]
fn when_computing_the_magnitodue_of_vector_0_1_0_you_get_1() {
    assert_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
}


#[rstest]
fn when_computing_the_magnitodue_of_vector_0_0_1_you_get_1() {
    assert_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
}

#[rstest]
fn when_computing_the_magnitodue_of_vector_1_2_3_you_get_sqrt_14() {
    assert_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), f32::sqrt(14.0));
}


#[rstest]
fn when_computing_the_magnitodue_of_vector_neg_1_neg_2_neg_3_you_get_sqrt_14() {
    assert_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), f32::sqrt(14.0));
}


#[rstest]
fn when_normalizing_vector_4_0_0_you_get_1_0_0() {
    assert_eq!(Vector::new(4.0, 0.0, 0.0).normalize(), Vector::new(1.0, 0.0, 0.0));
}

#[rstest]
fn when_normalizing_vector_1_2_3_you_get_0_26726_0_53452_0_00178() {
    assert_eq!(Vector::new(1.0, 2.0, 3.0).normalize(), Vector::new(0.26726, 0.53452, 0.80178));
}

#[rstest]
fn when_computing_dot_product_of_1_2_3_against_2_3_4_you_get_20() {
    assert_eq!(Vector::new(1.0, 2.0, 3.0).dot(Vector::new(2.0, 3.0, 4.0)), 20.0)
}

#[rstest]
fn when_computing_cross_of_1_2_3_against_2_3_4_you_get_neg_1_2_neg_1_and_opposite_cross_you_get_1_neg_2_1() {
    assert_eq!(
        Vector::new(1.0, 2.0, 3.0).cross(Vector::new(2.0, 3.0, 4.0)), Vector::new(-1.0, 2.0, -1.0)
    );

    assert_eq!(
        Vector::new(2.0, 3.0, 4.0).cross(Vector::new(1.0, 2.0, 3.0)), Vector::new(1.0, -2.0, 1.0)
    );
    
}

#[rstest]
fn when_adding_colors_you_get_a_color() {
    let C1 = Color::new(0.9, 0.6, 0.75);
    let C2 = Color::new(0.7, 0.1, 0.25);
    let C3 = C1 + C2;
    assert_eq!(C3, Color::new(1.6, 0.7, 1.0))
}

#[rstest]
fn when_sub_colors_you_get_a_color() {
    let C1 = Color::new(0.9, 0.6, 0.75);
    let C2 = Color::new(0.7, 0.1, 0.25);
    let C3 = C1 - C2;
    assert_eq!(C3, Color::new(0.2, 0.5, 0.5))
}

#[rstest]
fn when_mul_color_by_scalar_you_get_a_color() {
    let C1 = Color::new(0.2, 0.3, 0.4);
    let a = 2.0;
    let C2 = C1 * a;
    assert_eq!(C2, Color::new(0.4, 0.6, 0.8))
}

#[rstest]
fn when_mul_two_colors_you_get_a_color_correspondent_components_multiplied() {
    let C1 = Color::new(1.0, 0.2, 0.4);
    let C2 = Color::new(0.9, 1.0, 0.1);
    let C3 = C1 * C2;
    assert_eq!(C3, Color::new(0.9, 0.2, 0.04))
}