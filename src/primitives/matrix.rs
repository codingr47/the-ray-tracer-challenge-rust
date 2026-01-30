use std::ops::Index;

use crate::utils::math::equal;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f32; C]; R],
}

impl<const R: usize, const C: usize> From<[[f32; C]; R]> for Matrix<R, C> {
    fn from(data: [[f32; C]; R]) -> Self {
        Self { data }
    }
}
impl<const R:usize, const C: usize> Index<(usize, usize)> for Matrix<R, C> {
   type Output = f32;

   fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        return &self.data[row][col];
   } 
}

impl<const R: usize, const C:usize> Matrix<R, C> {
    pub fn get(&self, r: usize, c: usize) -> f32 {
        return self.data[r][c];
    } 

    pub fn set(&mut self, r:usize, c:usize, v: f32) {
        self.data[r][c] = v;
    }
}

 impl<const R: usize, const C:usize>  PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                if self[(x, y)] != other[(x, y)] {
                    return false
                }
            }
        }

        return true;
    }
}
