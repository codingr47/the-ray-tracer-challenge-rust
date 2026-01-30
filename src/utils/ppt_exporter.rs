use crate::primitives::{canvas::Canvas, color::Color};

pub struct PPTExporter<'a> {
    pixel_data: Vec<Vec<Color>>,  
    buffer: String,
    output: Vec<String>,
    canvas: &'a Canvas
}

impl<'a> PPTExporter<'a> {
    pub fn new(pixels: Vec<Vec<Color>>, canvas: &'a Canvas) -> Self {
        const MAGIC_NUMBER: usize = 70;
        let buffer = String::new();
        let output = Vec::<String>::new();
        return PPTExporter { pixel_data:  pixels, buffer: buffer, canvas, output };
    }

    pub fn flush(&mut self) {
         self.output.push(self.buffer.to_owned());
         self.buffer.clear();
    }
    pub fn append_value(&mut self, value: String, should_add_space_after: bool) {
         const MAGIC_NUMBER: usize = 70;
         let space_addition: usize = if should_add_space_after { 1 } else { 0 };
         if self.buffer.len() + value.len() + space_addition >= MAGIC_NUMBER {
             if self.buffer.as_bytes()[self.buffer.len() - 1] == b' ' {
                self.buffer = self.buffer[0..self.buffer.len() - 1].to_string();
             }
             self.flush();
             self.output.push("\n".to_string());
         }

         self.buffer += &value;
         if should_add_space_after {
            self.buffer += " ";
         }
    }

    fn append_value_direct(&mut self, value: String) {
        self.output.push(value);
    }

    pub fn export(&mut self) -> String {
        self.append_value_direct(format!("P3\n{} {}\n255\n", self.canvas.width.to_string(),  self.canvas.height.to_string()));
        for y in 0..self.canvas.height {
            for x in 0..self.canvas.width {
                if x > 0 && self.buffer.len() > 0 {
                    self.append_value(" ".to_string(), false);
                }
                let color_at = self.canvas.pixel_at(x, y);
                let color = color_at.unwrap();
                let r = i32::clamp((f32::round(color.red() * 255.0)) as i32, 0, 255).to_string();
                let g = i32::clamp((f32::round(color.green() * 255.0)) as i32, 0, 255).to_string();
                let b = i32::clamp((f32::round(color.blue() * 255.0)) as i32, 0, 255).to_string();
                self.append_value(r, true);
                self.append_value(g, true);
                self.append_value(b, false);
            }
            self.flush();
            self.append_value("\n".to_string(), false);
        }
        self.flush();

        return self.output.join("");
    }
}


