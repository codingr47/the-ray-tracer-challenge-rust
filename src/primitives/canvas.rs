use crate::primitives::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let mut canvas = Canvas {
            width: w,
            height: h,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); w as usize]; h as usize]
        };

        return canvas;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y).and_then(|row| row.get(x))
    }

    pub fn write_pixel(&mut self, x: usize, y:  usize, target: Color) {
       if let Some(row) = self.pixels.get_mut(y) {
        if let Some(pixel) = row.get_mut(x) {
            *pixel = target;
        }
       }
    }
}