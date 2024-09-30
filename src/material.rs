use crate::color::Color;
use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
    pub specular: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub has_texture: bool,
    pub texture: Option<DynamicImage>, // Agregar la textura
}

impl Material {
    pub fn new(diffuse: Color, albedo: f32, specular: f32, reflectivity: f32, transparency: f32, has_texture: bool, texture_path: Option<&str>) -> Self {
        let texture = if let Some(path) = texture_path {
            Some(image::open(path).expect("Failed to load texture"))
        } else {
            None
        };

        Material {
            diffuse,
            albedo,
            specular,
            reflectivity,
            transparency,
            has_texture,
            texture,
        }
    }
    pub fn get_color_from_texture(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            // Calcular las coordenadas de píxel dentro de la imagen de la textura
            let x = ((u * (texture.width() as f32)).min(texture.width() as f32 - 1.0)) as u32;
            let y = ((v * (texture.height() as f32)).min(texture.height() as f32 - 1.0)) as u32;
            
    
            if x < texture.width() && y < texture.height() {
                let pixel = texture.get_pixel(x, y);
                return Color::new(pixel[0], pixel[1], pixel[2]);
            }
        }
    
        // Si no hay textura, devuelve el color difuso
        self.diffuse
    }
    
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            albedo: 1.0,
            specular: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            has_texture: false,
            texture: None,
        }
    }
}
