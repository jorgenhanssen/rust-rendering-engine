extern crate nalgebra_glm as glm;

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

        m = glm::rotate(&m, self.rotation.x, &left());
        m = glm::rotate(&m, self.rotation.y, &up());
        m = glm::rotate(&m, self.rotation.z, &forward());

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

pub fn left() -> glm::Vec3 {
    return glm::vec3(-1.0, 0.0, 0.0);
}
pub fn right() -> glm::Vec3 {
    return glm::vec3(1.0, 0.0, 0.0);
}
pub fn up() -> glm::Vec3 {
    return glm::vec3(0.0, 1.0, 0.0);
}
pub fn down() -> glm::Vec3 {
    return glm::vec3(0.0, -1.0, 0.0);
}
pub fn forward() -> glm::Vec3 {
    return glm::vec3(0.0, 0.0, -1.0);
}
pub fn back() -> glm::Vec3 {
    return glm::vec3(0.0, 0.0, 1.0);
}