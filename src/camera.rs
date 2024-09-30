use nalgebra_glm::{Vec3, rotate_y, rotate_x, Mat4, Vec4};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    pub fn get_ray_direction(&self, u: f32, v: f32, aspect_ratio: f32) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let fov = 1.0;
        let x = (2.0 * u - 1.0) * aspect_ratio * fov;
        let y = (1.0 - 2.0 * v) * fov;

        (forward + right * x + up * y).normalize()
    }

    pub fn rotate(&mut self, angle_x: f32, angle_y: f32) {
        let direction = self.center - self.eye;
        
        let rotation_x = rotate_y(&Mat4::identity(), angle_x);
        let rotation_y = rotate_x(&Mat4::identity(), angle_y);

        // Convertir el vector 3D a 4D usando to_homogeneous
        let homogeneous_direction = direction.to_homogeneous();

        // Multiplicar por las matrices de rotación
        let new_direction = rotation_y * rotation_x * homogeneous_direction;

        // Convertir el resultado de nuevo a un vector 3D usando xyz()
        let new_direction_3d = Vec3::new(new_direction.x, new_direction.y, new_direction.z);

        self.eye = self.center - new_direction_3d;
    }
}
