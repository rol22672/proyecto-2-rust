use crate::color::Color;
use image::{RgbImage, Rgb};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0, 0, 0); width * height];
        Framebuffer { width, height, pixels }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn save(&self, filename: &str) {
        let mut img = RgbImage::new(self.width as u32, self.height as u32);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x];
                img.put_pixel(x as u32, y as u32, Rgb([color.r, color.g, color.b]));
            }
        }

        img.save(filename).expect("Failed to save image");
    }
}
