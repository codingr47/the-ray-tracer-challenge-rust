use crate::{primitives::color::Color, utils::ppt_exporter::PPTExporter};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>
}

type Byte = u8;

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

    
    pub fn to_ppm(&self) -> String {
        let mut publisher = PPTExporter::new(self.pixels.clone(), self);
        
        // const MAGIC_NUMBER: usize = 70;
        // let mut bytes = Vec::new();

        // bytes.push(format!("P3\n{} {}\n255\n", self.width.to_string(),  self.height.to_string()));
        // let mut current_line = String::new();
        // let mut append_value = |str_to_append: &str| {
        //   if current_line.len() + str_to_append.len() <= MAGIC_NUMBER {
        //      current_line.push_str(str_to_append);
        //   } else {
            
        //     if current_line.len() > 0 {
        //         bytes.push(current_line.clone());
        //     }

        //     current_line = str_to_append.to_string();
        //   }
        // };

        // let mut reset_current_line = || {
        //     current_line = String::new();
        // };
        
        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let color_at = self.pixel_at(x, y);
        //         let color = color_at.unwrap();
                
        //         if x > 0 {
        //             append_value(" ");
        //         }
        //         let r = i32::clamp((f32::round(color.red() * 255.0)) as i32, 0, 255).to_string() + " ";
        //         let g = i32::clamp((f32::round(color.green() * 255.0)) as i32, 0, 255).to_string() + " ";
        //         let b = i32::clamp((f32::round(color.blue() * 255.0)) as i32, 0, 255).to_string();
        //         append_value( &r);
        //         append_value( &g);
        //         append_value( &b);

        //     }
        //     append_value("");
        //     reset_current_line();
        //     append_value("\n");
        // }
        // return bytes.join("");

        return publisher.export();

    }
}