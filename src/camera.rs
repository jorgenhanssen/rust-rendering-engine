extern crate nalgebra_glm as glm;

use super::toolbox;
use super::scene_graph;

pub struct Camera {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
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

        m = glm::rotate(&m, self.rotation.y, &toolbox::up());
        m = glm::rotate(&m, self.rotation.x, &toolbox::left());
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


pub struct ChaseCamera {
    focus: glm::Vec3,
    offset: glm::Vec2,
    rotation: glm::Vec3,
}
impl ChaseCamera {
    pub fn new() -> ChaseCamera {
        return ChaseCamera{
            focus: glm::vec3(0.0, 0.0, 0.0),
            offset: glm::vec2(0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
        };
    }

    pub fn view(&mut self) -> glm::Mat4 {
        let mut m = glm::identity();

        m = glm::rotate(&m, self.rotation.x, &toolbox::left());
        m = glm::rotate(&m, self.rotation.z, &toolbox::forward());

        m = glm::translate(&m, &glm::vec3(0.0, 0.0, self.offset.y));

        m = glm::rotate(&m, -self.rotation.y, &toolbox::up());

        m = glm::translate(&m, &glm::vec3(0.0, self.offset.x, 0.0));

        m = glm::translate(&m, &-self.focus);


        return m;
    }

    pub fn set_offset(&mut self, offset: glm::Vec2) {
        self.offset = offset;
    }
    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
    }

    pub fn chase(&mut self, subject_position: glm::Vec3, subject_rotation: glm::Vec3, softness: f32) {
        let camera_pos_diff = subject_position - self.focus;
        self.focus += camera_pos_diff / softness;

        let mut camera_rot_diff = subject_rotation.y - self.rotation.y;
        camera_rot_diff = (camera_rot_diff + 180.0) % 360.0 - 180.0;

        self.rotation.y += camera_rot_diff / (1.5 * softness);
    }
}
