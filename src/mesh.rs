use tobj;

const POSITION_SIZE: i32 = 3;
const NORMAL_SIZE: i32 = 3;
const COLOR_SIZE: i32 = 4;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: glm::Vec3,
    normal: glm::Vec3,
    color:  glm::Vec4,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub index_count: i32,

    vao: u32,
}

impl Mesh {
    pub fn from(mesh: tobj::Mesh, color: [f32; 4]) -> Self {
        let mut vertices: Vec<Vertex> = vec![];
        let num_verts = mesh.positions.len() / 3;
        let index_count = mesh.indices.len() as i32;

        for i in 0..num_verts {
            let vp = i * 3; // vertex pointer
    
            vertices.push(Vertex{
                position: glm::vec3(mesh.positions[vp], mesh.positions[vp+1], mesh.positions[vp+2]),
                normal: glm::vec3(mesh.normals[vp], mesh.normals[vp+1], mesh.normals[vp+2]),
                color: glm::vec4(color[0], color[1], color[2], color[3]),
            })
        }

        let mut m = Mesh {
            vertices: vertices,
            indices: mesh.indices,
            index_count,
            vao: 0,
        };

        m.compute_vao();

        return m
    }

    fn compute_vao(&mut self) {
        // vertices contains a description of all triangles in the mesh
        // so that every set of 3 vertices creates a triangle. This set will, therefore,
        // contain duplicates of vertices shared by some triangles.
        let mut vertices: Vec<Vertex> = vec![];
        for i in self.indices.iter() {
            vertices.push(self.vertices[*i as usize]);
        }
        
        // unpacked_vertices contains all vertices from but
        // unpacked into [..., ...vec_i.position, ...vec_i.colors, ...]
        let unpacked_vertices = unpack_vertices(vertices);

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        unsafe {
            // Set up VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes(&unpacked_vertices), to_ptr(&unpacked_vertices), gl::STATIC_DRAW);

            // Set up VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // set up index buffer
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_in_bytes(&self.indices), to_ptr(&self.indices), gl::STATIC_DRAW);


            let stride_size: i32 = POSITION_SIZE + COLOR_SIZE + NORMAL_SIZE;

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                POSITION_SIZE,
                gl::FLOAT,
                gl::FALSE,
                (stride_size as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                COLOR_SIZE,
                gl::FLOAT,
                gl::FALSE,
                (stride_size as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                (POSITION_SIZE as usize * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                NORMAL_SIZE,
                gl::FLOAT,
                gl::FALSE,
                (stride_size as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((POSITION_SIZE + COLOR_SIZE) as usize * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );
        }

        self.vao = vao;
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, self.indices.len() as i32);
        gl::BindVertexArray(0);
    }
}

pub struct Terrain;
impl Terrain {
    pub fn load(path: &str) -> Mesh {
        println!("Loading terrain model...");
        let before = std::time::Instant::now();
        let (models, _materials) = tobj::load_obj(path, true).expect("Failed to load terrain model");
        let after = std::time::Instant::now();
        println!("Done in {:.3}ms.", after.duration_since(before).as_micros() as f32 / 1e3);
        if models.len() > 1 || models.len() == 0 { panic!("Please use a model with a single mesh") }

        let terrain = models[0].to_owned();
        println!("Loaded {} with {} points and {} triangles.", terrain.name, terrain.mesh.positions.len() /3, terrain.mesh.indices.len() / 3);

        Mesh::from(terrain.mesh, [1.0, 1.0, 1.0, 1.0])
    }
}

use std::ops::Index;
pub struct Helicopter {
    pub body: Mesh,
    pub main_rotor: Mesh,
    pub tail_rotor: Mesh,
    pub door: Mesh,
}

// You can use square brackets to access the components of the helicopter, if you want to use loops!
impl Index<usize> for Helicopter {
    type Output = Mesh;
    fn index<'a>(&'a self, i: usize) -> &'a Mesh {
        match i {
            0 => &self.body,
            1 => &self.main_rotor,
            2 => &self.tail_rotor,
            3 => &self.door,
            _ => panic!("Invalid index, try [0,3]"),
        }
    }
}

impl Helicopter {
    pub fn load(path: &str) -> Self {
        println!("Loading helicopter model...");
        let before = std::time::Instant::now();
        let (models, _materials) = tobj::load_obj(path, true).expect("Failed to load helicopter model");
        let after = std::time::Instant::now();
        println!("Done in {:.3}ms.", after.duration_since(before).as_micros() as f32 / 1e3);

        for model in &models {
            println!("Loaded {} with {} points and {} triangles.", model.name, model.mesh.positions.len() / 3, model.mesh.indices.len() / 3);
        }

        let body_model = models.iter().find(|m| m.name == "Body_body").expect("Incorrect model file!").to_owned();
        let main_rotor_model = models.iter().find(|m| m.name == "Main_Rotor_main_rotor").expect("Incorrect model file!").to_owned();
        let tail_rotor_model = models.iter().find(|m| m.name == "Tail_Rotor_tail_rotor").expect("Incorrect model file!").to_owned();
        let door_model = models.iter().find(|m| m.name == "Door_door").expect("Incorrect model file!").to_owned();

        Helicopter {
            body:       Mesh::from(body_model.mesh,         [0.3, 0.3, 0.3, 1.0]),
            main_rotor: Mesh::from(main_rotor_model.mesh,   [0.3, 0.1, 0.1, 1.0]),
            tail_rotor: Mesh::from(tail_rotor_model.mesh,   [0.1, 0.3, 0.1, 1.0]),
            door:       Mesh::from(door_model.mesh,         [0.1, 0.1, 0.3, 1.0]),
        }
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
fn unpack_vertices(vec: Vec<Vertex>) -> Vec<f32> {
    let mut unpacked: Vec<f32> = vec![];

    for v in vec.iter() {
        unpacked.append(&mut vec![
            v.position.x, v.position.y, v.position.z,
            v.color.x, v.color.y, v.color.z, v.color.w,
            v.normal.x, v.normal.y, v.normal.z,
        ]);
    }

    unpacked
}
