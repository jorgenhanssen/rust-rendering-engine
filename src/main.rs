// use gl::types::*;
use std::{
    // mem,
    ptr,
    // str,
    // os::raw::c_void,
};
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

extern crate nalgebra_glm as glm;

mod shader;
mod util;
mod camera;
mod mesh;
mod scene_graph;
mod toolbox;

use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 1920;
const SCREEN_H: u32 = 1080;

const ROTATION_SPEED_MULTIPLIER: f32 = 0.4;
const CAMERA_SOFTNESS: f32 = 80.0;


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Rust Rendering Engine")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    
    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Send a copy of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the renderin thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        let program_id: u32;
        unsafe {
            // misc boilerplate
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            // Set up shaders
            let shaders = shader::ShaderBuilder::new()
            .attach_file("shaders/simple.vert")
            .attach_file("shaders/simple.frag")
            .link();

            program_id = shaders.program_id;
            gl::UseProgram(program_id);
        }
    
        let projection: glm::Mat4 = glm::perspective(SCREEN_W  as f32 / SCREEN_H as f32, 1.0, 1.0, 2000.0);

        let mut camera = camera::ChaseCamera::new();
        camera.set_offset(glm::vec2(-8.0, -24.0));
        camera.set_rotation(glm::vec3(-0.35, 0.0, 0.0));


        // load models
        let terrain_model = mesh::Terrain::load("resources/lunarsurface.obj");
        let helicopter_model = mesh::Helicopter::load("resources/helicopter.obj");

        // setup scene graph
        let mut root = scene_graph::SceneNode::new();

        // Create tarrain node and attach it to root
        let mut terrain_node = scene_graph::SceneNode::from_mesh(&terrain_model);
        root.add_child(&terrain_node);

        let mut helicopter = Helicopter::new(&helicopter_model, &mut terrain_node);


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        loop {
            let now = std::time::Instant::now();
            // let elapsed = now.duration_since(first_frame_time).as_secs_f32(); // not in use yet.
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // Translation
                        VirtualKeyCode::A => {
                            helicopter.rotate(toolbox::up() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::D => {
                            helicopter.rotate(toolbox::down() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::W => {
                            helicopter.throttle(delta_time);
                        },
                        VirtualKeyCode::S => {
                            helicopter.throttle(-delta_time);
                        },
                        VirtualKeyCode::Up => {
                            helicopter.rotate(toolbox::right() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Down => {
                            helicopter.rotate(toolbox::left() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Left => {
                            helicopter.rotate(toolbox::forward() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Right => {
                            helicopter.rotate(toolbox::back() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }

                        VirtualKeyCode::PageUp => {
                            helicopter.toggle_door(true);
                        }
                        VirtualKeyCode::PageDown => {
                            helicopter.toggle_door(false);
                        }

                        _ => { }
                    }
                }
            }

            helicopter.frame(delta_time);

            camera.chase(
                helicopter.get_position(), 
                helicopter.get_rotation(), 
                CAMERA_SOFTNESS / (glm::length(&helicopter.get_speed()) + 1.0)
            );

            let view_projection: glm::Mat4 = projection * camera.view();

            // Drawing
            unsafe {
                gl::ClearColor(0.163, 0.163, 0.163, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                root.update_transformations(&glm::identity());
                root.draw(program_id, &view_projection);
            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => { }
                }
            },
            _ => { }
        }
    });
}



struct Helicopter {
    root: scene_graph::Node,
    body: scene_graph::Node,
    main_rotor: scene_graph::Node,
    tail_rotor: scene_graph::Node,
    door: scene_graph::Node,

    speed: glm::Vec3,
    throttle: f32,

    door_open: bool
}
impl Helicopter {
    pub fn new(helicopter: &mesh::Helicopter, parent: &mut scene_graph::Node) -> Helicopter {
        let mut h = Helicopter{
            root: scene_graph::SceneNode::new(),
            body: scene_graph::SceneNode::from_mesh(&helicopter.body),
            main_rotor: scene_graph::SceneNode::from_mesh(&helicopter.main_rotor),
            tail_rotor: scene_graph::SceneNode::from_mesh(&helicopter.tail_rotor),
            door: scene_graph::SceneNode::from_mesh(&helicopter.door),

            speed: glm::vec3(0.0, 0.0, 0.0),
            throttle: 0.0,

            door_open: false
        };

        // Node adjustments
        h.tail_rotor.reference_point = glm::vec3(0.35, 2.3, 10.4);

        // Bind graph
        h.root.add_child(&h.body);
        h.body.add_child(&h.door);
        h.body.add_child(&h.main_rotor);
        h.body.add_child(&h.tail_rotor);

        parent.add_child(&h.root);

        return h;
    }

    pub fn get_rotation(&self) -> glm::Vec3 {
        return self.body.rotation;
    }
    pub fn get_position(&self) -> glm::Vec3 {
        return self.root.position;
    }
    pub fn get_speed(&self) -> glm::Vec3 {
        return self.speed;
    }

    pub fn toggle_door(&mut self, open: bool) {
        self.door_open = open;
    }

    pub fn throttle(&mut self, value: f32) {
        if (value + self.throttle).abs() < 0.5 {
            self.throttle += value;
        }
    }

    pub fn rotate(&mut self, rotation: glm::Vec3) {
        self.body.rotation += rotation;
    }

    pub fn frame(&mut self, delta_time: f32) {
        self.main_rotor.rotation += glm::vec3(0.0, delta_time*(self.throttle+1.0)*25.0, 0.0);
        self.tail_rotor.rotation += glm::vec3(delta_time*20.0, 0.0, 0.0);

        self.speed = self.speed + glm::rotate_y_vec3(&glm::vec3(
            0.2 * self.body.rotation.z * delta_time, 
            0.2 * self.throttle * delta_time,
            -0.2 * self.body.rotation.x * delta_time, 
        ), self.body.rotation.y);

        self.root.position += self.speed;



        // Animate the door - z direction is boring, but works
        let mut door_diff = 0.0;
        if self.door_open {
            door_diff = 2.0;
        }

        door_diff -= self.door.position.z;
        self.door.position.z += door_diff * delta_time;
   }    
}