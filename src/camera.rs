extern crate nalgebra_glm as glm;

use super::toolbox;

pub struct Camera {
    position: glm::Vec3,
    rotation: glm::Vec3,
}
impl Camera {
    pub fn new() -> Camera {
        return Camera{
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
        };
    }

    pub fn view(&mut self) -> glm::Mat4 {
        let mut m = glm::identity();

        m = glm::rotate(&m, self.rotation.x, &toolbox::left());
        m = glm::rotate(&m, self.rotation.y, &toolbox::up());
        m = glm::rotate(&m, self.rotation.z, &toolbox::forward());

        m = glm::translate(&m, &self.position);

        return m;
    }

    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
    }
    pub fn rotate(&mut self, rotation: glm::Vec3) {
        self.rotation += rotation;
    }

    pub fn set_position(&mut self, position: glm::Vec3) {
        self.position = position;
    }
    pub fn translate(&mut self, translation: glm::Vec3) {
        self.position += translation;
    }
}
