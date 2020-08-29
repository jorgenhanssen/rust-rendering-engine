extern crate nalgebra_glm as glm;

pub struct Renderable {
    vertices: Vec<glm::Vec3>,
    indices: Vec<u32>,
}

impl Renderable {
    // vao creates a VAO (vertex array object) of the Renderable and returns the ID.
    pub fn vao(&self) -> (u32, i32) {
        // triangle_vertices contains a description of all triangles in the mesh
        // so that every set of 3 vertices creates a triangle. This set will, therefore,
        // contain duplicates of vertices shared by some triangles.
        let mut triangle_vertices: Vec<glm::Vec3> = vec![];
        for i in self.indices.iter() {
            triangle_vertices.push(self.vertices[*i as usize]);
        }

        // unpacked_vertices contains all vertices from triangle_vertices but
        // unpacked into [vec1.x, vec1.y, vec1.z, vec2.x, vec2.y, ...]
        let unpacked_vertices = unpack_vertices(triangle_vertices);
        
        let (mut vbo, mut vao) = (0, 0);
        unsafe {
            // Set up VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes(&unpacked_vertices), to_ptr(&unpacked_vertices), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Set up VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
        }

        return (vao, unpacked_vertices.len() as i32)
    }
}









// Helper functions ========================================================

// size_in_bytes returns the size of a vector in GL bytes
fn size_in_bytes<T>(v: &Vec<T>) -> gl::types::GLsizeiptr {
    (v.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr
}

// to_ptr creates a GL ptr to a vector of T.
fn to_ptr<T>(v: &Vec<T>) -> *const gl::types::GLvoid {
    v.as_ptr() as *const gl::types::GLvoid
}

// unpack_vertices unpacks a collection of Vector3 vertices to a collection of floats.
fn unpack_vertices(vertices: Vec<glm::Vec3>) -> Vec<f32> {
    let mut unpacked: Vec<f32> = vec![];

    for v in vertices.iter() {
        unpacked.append(&mut vec![
            v.x, v.y, v.z
        ]);
    }

    unpacked
}







// Some fun shapes to test with ==============================================
// TODO: Remove these

pub fn triangle() -> Renderable{
    Renderable{
        // coordinates from assignment
        vertices: vec![
            glm::vec3(-0.6, -0.6, 0.0),
            glm::vec3(0.6, -0.6, 0.0),
            glm::vec3(0.0, 0.6, 0.0),
        ],
        indices: vec!(0, 1, 2)
    }
}

pub fn square() -> Renderable{
    Renderable{
        vertices: vec![
            glm::vec3(-0.6, -0.6, 0.0), // bottom-left
            glm::vec3(0.6, -0.6, 0.0),  // bottom-right
            glm::vec3(-0.6, 0.6, 0.0),  // top-left
            glm::vec3(0.6, 0.6, 0.0),   // top-right
        ],
        indices: vec!(
            0, 1, 2,
            2, 1, 3
        )
    }
}

pub fn circle(resolution: i32) -> Renderable{
    let step_size: f32 = 2.0 * std::f32::consts::PI / resolution as f32;

    let mut vertices: Vec<glm::Vec3> = vec![glm::vec3(0.0, 0.0, 0.0)];
    for i in 0..resolution {
        let v = (i as f32)*step_size;
        vertices.push(glm::vec3(v.cos(), v.sin(), 0.0))
    }

    let mut indices: Vec<u32> = vec![];
    for i in 0..resolution {
        indices.push(0); indices.push(i as u32); indices.push((i+1) as u32);
    }

    // We need this last one to complete the circle
    indices.push(0); indices.push((vertices.len()-1) as u32); indices.push(1);

    Renderable{
        vertices: vertices,
        indices: indices,
    }
}