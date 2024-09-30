use nalgebra_glm::Vec3;
use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use crate::color::Color;
use crate::material::Material;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        Cube { center, size, material }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let half_size = self.size / 2.0;

        // Define the cube boundaries
        let min_bound = self.center - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center + Vec3::new(half_size, half_size, half_size);

        // Calculate intersection with the cube
        let mut t_min = (min_bound.x - ray_origin.x) / ray_direction.x;
        let mut t_max = (max_bound.x - ray_origin.x) / ray_direction.x;
        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut ty_min = (min_bound.y - ray_origin.y) / ray_direction.y;
        let mut ty_max = (max_bound.y - ray_origin.y) / ray_direction.y;
        if ty_min > ty_max {
            std::mem::swap(&mut ty_min, &mut ty_max);
        }

        if (t_min > ty_max) || (ty_min > t_max) {
            return Intersect::empty();
        }

        if ty_min > t_min {
            t_min = ty_min;
        }
        if ty_max < t_max {
            t_max = ty_max;
        }

        let mut tz_min = (min_bound.z - ray_origin.z) / ray_direction.z;
        let mut tz_max = (max_bound.z - ray_origin.z) / ray_direction.z;
        if tz_min > tz_max {
            std::mem::swap(&mut tz_min, &mut tz_max);
        }

        if (t_min > tz_max) || (tz_min > t_max) {
            return Intersect::empty();
        }

        if tz_min > t_min {
            t_min = tz_min;
        }
        if tz_max < t_max {
            t_max = tz_max;
        }

        // Calculate intersection point
        let intersection_distance = t_min;
        let intersection_point = ray_origin + ray_direction * intersection_distance;

        // Determine which face is hit and calculate UV coordinates accordingly
        let mut u = 0.0;
        let mut v = 0.0;
// Determina la cara intersectada y calcula las coordenadas UV correctamente
if (intersection_point.x - min_bound.x).abs() < 1e-4 {
    u = (intersection_point.z - min_bound.z) / (max_bound.z - min_bound.z);
    v = (intersection_point.y - min_bound.y) / (max_bound.y - min_bound.y);
} else if (intersection_point.x - max_bound.x).abs() < 1e-4 {
    u = (intersection_point.z - min_bound.z) / (max_bound.z - min_bound.z);
    v = (intersection_point.y - min_bound.y) / (max_bound.y - min_bound.y);
} else if (intersection_point.y - min_bound.y).abs() < 1e-4 {
    u = (intersection_point.x - min_bound.x) / (max_bound.x - min_bound.x);
    v = (intersection_point.z - min_bound.z) / (max_bound.z - min_bound.z);
} else if (intersection_point.y - max_bound.y).abs() < 1e-4 {
    u = (intersection_point.x - min_bound.x) / (max_bound.x - min_bound.x);
    v = (intersection_point.z - min_bound.z) / (max_bound.z - min_bound.z);
} else if (intersection_point.z - min_bound.z).abs() < 1e-4 {
    u = (intersection_point.x - min_bound.x) / (max_bound.x - min_bound.x);
    v = (intersection_point.y - min_bound.y) / (max_bound.y - min_bound.y);
} else if (intersection_point.z - max_bound.z).abs() < 1e-4 {
    u = (intersection_point.x - min_bound.x) / (max_bound.x - min_bound.x);
    v = (intersection_point.y - min_bound.y) / (max_bound.y - min_bound.y);
}

// Ajusta los valores de UV para asegurar que están en el rango de [0, 1]
u = u.clamp(0.0, 1.0);
v = 1.0 - v; // Invertir v para adaptarse al sistema de coordenadas de textura

        // Get the color from the texture if available
        let color = if self.material.has_texture {
            self.material.get_color_from_texture(u, v)
        } else {
            self.material.diffuse
        };

        Intersect::new(intersection_point, Vec3::new(0.0, 0.0, 0.0), intersection_distance, &self.material, u, v)
    }
}


