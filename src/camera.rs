extern crate nalgebra_glm;

use nalgebra_glm as glm;

pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,

    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),

            fov: 45.0,
        }
    }

    pub fn calculate_view(&self) -> glm::Mat4 {
        glm::one()
    }

    pub fn calculate_projection(&self, window_size: (u32, u32)) -> glm::Mat4 {
        let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
        glm::perspective(aspect_ratio, self.fov.to_radians(), 0.1, 100.0)
    }
}
