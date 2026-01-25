use crate::primitives::{canvas::Canvas, color::Color};

pub struct PPTExporter<'a> {
    pixel_data: Vec<Vec<Color>>,  
    buffer: Vec<String>,
    output: Vec<String>,
    canvas: &'a Canvas
}

impl<'a> PPTExporter<'a> {
    pub fn new(pixels: Vec<Vec<Color>>, canvas: &'a Canvas) -> Self {
        const MAGIC_NUMBER: usize = 70;
        let buffer = Vec::<String>::new();
        let output = Vec::<String>::new();
        return PPTExporter { pixel_data:  pixels, buffer: buffer, canvas, output };
    }

    pub fn flush(&mut self) {
         self.output.push(self.buffer.join(""));
         self.buffer.clear();
    }
    pub fn append_value(&mut self, value: String) {
         const MAGIC_NUMBER: usize = 70;
         if self.buffer.len() + value.len() <= MAGIC_NUMBER {
             self.buffer.push(value);
          } else {
            self.output.push("\n".to_string());
            self.flush();
          }
    }

    pub fn export(&mut self) -> String {
        self.append_value(format!("P3\n{} {}\n255\n", self.canvas.width.to_string(),  self.canvas.height.to_string()));
        for y in 0..self.canvas.height {
            for x in 0..self.canvas.width {
                let color_at = self.canvas.pixel_at(x, y);
                let color = color_at.unwrap();
                let r = i32::clamp((f32::round(color.red() * 255.0)) as i32, 0, 255).to_string() + " ";
                let g = i32::clamp((f32::round(color.green() * 255.0)) as i32, 0, 255).to_string() + " ";
                let b = i32::clamp((f32::round(color.blue() * 255.0)) as i32, 0, 255).to_string() + " ";
                self.append_value(r);
                self.append_value(g);
                self.append_value(b);
            }
        }
        self.append_value("\n".to_string());
        self.flush();

        return self.output.join("");
    }
}


