use crate::material::Material;
use nalgebra_glm::Vec3;


#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: &Material, u: f32, v: f32) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material: material.clone(), // Clonamos el material para evitar moverlo
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material::default(),
            u: 0.0, // Valor por defecto
            v: 0.0, // Valor por defecto
        }
    }
}
