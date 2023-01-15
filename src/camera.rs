extern crate nalgebra_glm;

use nalgebra_glm as glm;

pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    velocity: glm::Vec3,
    fov: f32,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            right: glm::vec3(1.0, 0.0, 0.0),
            velocity: glm::vec3(0.0, 0.0, 0.0),
            fov: 45.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn calculate_view(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn calculate_projection(&self, window_size: (u32, u32)) -> glm::Mat4 {
        glm::perspective(
            window_size.0 as f32 / window_size.1 as f32,
            self.fov.to_radians(),
            0.1,
            1000.0,
        )
    }

    pub fn move_position(
        &mut self,
        acceleration: glm::Vec3,
        drag: f32,
        seconds_since_last_update: f32,
    ) {
        self.velocity *= drag;
        self.velocity += seconds_since_last_update * acceleration;
        self.position += seconds_since_last_update * self.velocity.x * self.right;
        self.position += seconds_since_last_update * self.velocity.y * self.up;
        self.position += seconds_since_last_update * self.velocity.z * self.front;
    }

    pub fn move_orientation(&mut self, x_rel: f32, y_rel: f32) {
        self.yaw += x_rel;
        self.pitch = (self.pitch - y_rel).clamp(-89.0, 89.0);
        self.front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        self.right = glm::normalize(&glm::cross(&self.front, &self.up));
    }

    pub fn zoom(&mut self, offset: f32, seconds_since_last_update: f32) {
        self.fov = (self.fov - offset * seconds_since_last_update).clamp(1.0, 75.0);
    }
}
