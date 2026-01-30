use rstest::rstest;
use the_raytracer_challenge_rust::primitives::{matrix::Matrix};


#[rstest]
fn when_constructing_a_4_by_4_matrix_values_are_accessible() {
    let m = Matrix::from([[1.0, 2.0, 4.0, 4.0], 
                                        [5.5, 6.5, 7.5, 8.5], 
                                        [9.0, 10.0, 11.0, 12.0], 
                                        [13.5, 14.5, 15.5, 16.5]
                                      ]);
    assert_eq!(m.get(0, 0), 1.0);
    assert_eq!(m.get(0, 3), 4.0);
    assert_eq!(m.get(1, 0), 5.5);
    assert_eq!(m.get(1, 2), 7.5);
    assert_eq!(m.get(2, 2), 11.0);
    assert_eq!(m.get(3, 0), 13.5);
    assert_eq!(m.get(3, 2), 15.5);
}

#[rstest]
fn when_constructing_a_2_by_2_matrix_values_are_accessible() {
    let m = Matrix::from([[-3.0, 5.0], 
                                        [1.0, -2.0], 
                                       ]);
    assert_eq!(m.get(0, 0), -3.0);
    assert_eq!(m.get(0, 1), 5.0);
    assert_eq!(m.get(1, 0), 1.0);
    assert_eq!(m.get(1, 1), -2.0);
}

#[rstest]
fn when_constructing_a_3_by_3_matrix_values_are_accessible() {
    let m = Matrix::from([[-3.0, 5.0, 0.0], 
                                        [1.0, -2.0, -7.0],
                                        [0.0, 1.0, 1.0] 
                                       ]);
    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(1, 1)], -2.0);
    assert_eq!(m[(2, 2)], 1.0);
}

#[rstest]
fn  when_matrix_A_and_matrix_B_are_identical_in_values_they_are_equal() {
  let A = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0]
  ]);
  let B = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0]
  ]);

  assert_eq!(A == B, true);
}

#[rstest]
fn  when_matrix_A_and_matrix_B_are_different_in_values_they_are_equal() {
  let A = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0]
  ]);
  let B = Matrix::from([
      [0.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0]
  ]);

  assert_eq!(A != B, true);
}