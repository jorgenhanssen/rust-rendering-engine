extern crate nalgebra_glm as glm;

use std::mem::ManuallyDrop;
use std::pin::Pin;
use super::mesh;
use super::toolbox;

// Used to crete an unholy abomination upon which you should not cast your gaze.
// This ended up being a necessity due to wanting to keep the code written by students as "straight forward" as possible
// It is very very double plus ungood Rust, and intentionally leaks memory like a sieve. But it works, and you're more than welcome to pretend it doesn't exist!
// In case you're curious about how it works; It allocates memory on the heap (Box), promises to prevent it from being moved or deallocated until dropped (Pin) 
// and finally prevents the compiler from dropping it automatically at all (ManuallyDrop). If that sounds like a janky solution, it's because it is.
// Prettier, Rustier and better solutions were tried numerous times, but were all found wanting of having what I sarbitrarily decided to be the required level of
// simplicity of use.
pub type Node = ManuallyDrop<Pin<Box<SceneNode>>>;

pub struct SceneNode {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub reference_point: glm::Vec3,

    pub current_transformation_matrix: glm::Mat4,

    pub mesh: Option<*mut mesh::Mesh>,

    pub children: Vec<*mut SceneNode>,
}

impl SceneNode {
    pub fn new() -> Node {
        ManuallyDrop::new(Pin::new(Box::new(SceneNode {
            position: glm::zero(),
            rotation: glm::zero(),
            scale: glm::vec3(1.0, 1.0, 1.0),
            reference_point: glm::zero(),
            current_transformation_matrix: glm::identity(),
            children: vec![],

            mesh: None,
        })))
    }
    pub fn from_mesh(m: &mesh::Mesh) -> Node {
        ManuallyDrop::new(Pin::new(Box::new(SceneNode {
            position: glm::zero(),
            rotation: glm::zero(),
            scale: glm::vec3(1.0, 1.0, 1.0),
            reference_point: glm::zero(),
            current_transformation_matrix: glm::identity(),
            children: vec![],

            mesh: Some(m as *const mesh::Mesh as *mut mesh::Mesh),
        })))
    }
    pub fn add_child(&mut self, child: &SceneNode) {
        self.children.push(child as *const SceneNode as *mut SceneNode)
    }
    pub fn print(&self) {
        let m = self.current_transformation_matrix;
        let matrix_string = format!(
"
      {:.2}  {:.2}  {:.2}  {:.2}
      {:.2}  {:.2}  {:.2}  {:.2}
      {:.2}  {:.2}  {:.2}  {:.2}
      {:.2}  {:.2}  {:.2}  {:.2}
",
            m[0],m[4],m[8],m[12],
            m[1],m[5],m[9],m[13],
            m[2],m[6],m[10],m[14],
            m[3],m[7],m[11],m[15],
        );
        println!(
"SceneNode {{
    Children:  {}
    Position:  [{:.2}, {:.2}, {:.2}]
    Rotation:  [{:.2}, {:.2}, {:.2}]
    Reference: [{:.2}, {:.2}, {:.2}]
    Current Transformation Matrix: {}
}}",
            self.children.len(),
            self.position.x,
            self.position.y,
            self.position.z,
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
            self.reference_point.x,
            self.reference_point.y,
            self.reference_point.z,
            matrix_string,
        );
    }

    pub unsafe fn draw(&self, program_id: u32, view_projection_matrix: &glm::Mat4) {
        // Check if node is drawable, set uniforms, draw
        if self.mesh != None { 
            // Set MVP uniform variable
            let mvp_name = "mvp";
            let mvp_id = gl::GetUniformLocation(program_id, std::ffi::CString::new(mvp_name).unwrap().as_ptr());

            let mvp: glm::Mat4 = view_projection_matrix * self.current_transformation_matrix;
            gl::UniformMatrix4fv(mvp_id, 1, gl::FALSE, mvp.as_ptr() as *const _);

            // Set Model uniform variable
            let model_matrix_name = "model";
            let model_matrix_id = gl::GetUniformLocation(program_id, std::ffi::CString::new(model_matrix_name).unwrap().as_ptr());

            let model_matrix: glm::Mat4 = self.current_transformation_matrix;
            gl::UniformMatrix4fv(model_matrix_id, 1, gl::FALSE, model_matrix.as_ptr() as *const _);
            
            // Draw the mesh
            let m: *mut mesh::Mesh = self.mesh.unwrap();
            &(*m).draw();
        }

        // Recurse
        for &child in &self.children {
            &(*child).draw(program_id, view_projection_matrix);
        }
    }

    pub unsafe fn update_transformations(&mut self, transformation_so_far: &glm::Mat4) {
        let mut m = *transformation_so_far;

        // Move model to center
        m = glm::translate(&m, &self.reference_point);

        // Rotate
        m = glm::rotate(&m, self.rotation.x, &toolbox::left());
        m = glm::rotate(&m, self.rotation.y, &toolbox::up());
        m = glm::rotate(&m, self.rotation.z, &toolbox::forward());

        // Scale
        m = glm::scale(&m, &self.scale);

        // Move model away from centers
        m = glm::translate(&m, &-self.reference_point);

        // Apply translation
        m = glm::translate(&m, &self.position);

        // Set transformations
        self.current_transformation_matrix = m;

        // Recurse
        for &child in &self.children {
            &(*child).update_transformations(&m);
        }
    }
}