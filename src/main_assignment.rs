// // use gl::types::*;
// use std::{
//     // mem,
//     ptr,
//     // str,
//     // os::raw::c_void,
// };
// use std::thread;
// use std::sync::{Mutex, Arc, RwLock};

// extern crate nalgebra_glm as glm;

// mod shader;
// mod util;
// mod camera;
// mod mesh;
// mod scene_graph;
// mod toolbox;

// use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
// use glutin::event_loop::ControlFlow;

// const SCREEN_W: u32 = 800;
// const SCREEN_H: u32 = 600;

// const TRANSLATION_SPEED_MULTIPLIER: f32 = 30.0;
// const ROTATION_SPEED_MULTIPLIER: f32 = 1.0;


// fn main() {
//     // Set up the necessary objects to deal with windows and event handling
//     let el = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new()
//         .with_title("Rust Rendering Engine")
//         .with_resizable(false)
//         .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
//     let cb = glutin::ContextBuilder::new()
//         .with_vsync(true);
//     let windowed_context = cb.build_windowed(wb, &el).unwrap();
    
//     // Set up a shared vector for keeping track of currently pressed keys
//     let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
//     // Send a copy of this vector to send to the render thread
//     let pressed_keys = Arc::clone(&arc_pressed_keys);

//     // Spawn a separate thread for rendering, so event handling doesn't block rendering
//     let render_thread = thread::spawn(move || {
//         // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the renderin thread, because
//         // an active OpenGL context cannot safely traverse a thread boundary
//         let context = unsafe {
//             let c = windowed_context.make_current().unwrap();
//             gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
//             c
//         };

//         // Set up openGL
//         let program_id: u32;
//         unsafe {
//             // misc boilerplate
//             gl::Enable(gl::CULL_FACE);
//             gl::Disable(gl::MULTISAMPLE);
//             gl::Enable(gl::BLEND);
//             gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
//             gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
//             gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());
//             gl::Enable(gl::DEPTH_TEST);
//             gl::DepthFunc(gl::LESS);
            
//             // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

//             // Set up shaders
//             let shaders = shader::ShaderBuilder::new()
//             .attach_file("shaders/simple.vert")
//             .attach_file("shaders/simple.frag")
//             .link();

//             program_id = shaders.program_id;
//             gl::UseProgram(program_id);
//         }
    
//         let projection: glm::Mat4 = glm::perspective(SCREEN_W  as f32 / SCREEN_H as f32, 1.0, 1.0, 2000.0);
//         let mut camera = camera::Camera::new();



//         // load models
//         let terrain = mesh::Terrain::load("resources/lunarsurface.obj");
//         let helicopter = mesh::Helicopter::load("resources/helicopter.obj");

//         // setup scene graph
//         let mut root = scene_graph::SceneNode::new();

//         // Create tarrain node and attach it to root
//         let mut terrain_node = scene_graph::SceneNode::from_mesh(&terrain);
//         root.add_child(&terrain_node);

//         for _ in 0..5 {
//             let mut helicopter_node = scene_graph::SceneNode::new();
//             build_helicopter(&helicopter, &mut helicopter_node);
//             terrain_node.add_child(&helicopter_node);
//         }

//         // The main rendering loop
//         let first_frame_time = std::time::Instant::now();
//         let mut last_frame_time = first_frame_time;

//         loop {
//             let now = std::time::Instant::now();
//             let elapsed = now.duration_since(first_frame_time).as_secs_f32(); // not in use yet.
//             let delta_time = now.duration_since(last_frame_time).as_secs_f32();
//             last_frame_time = now;

//             // Handle keyboard input
//             if let Ok(keys) = pressed_keys.lock() {
//                 for key in keys.iter() {
//                     match key {
//                         // Translation
//                         VirtualKeyCode::A => {
//                             camera.translate(-toolbox::left() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },
//                         VirtualKeyCode::D => {
//                             camera.translate(-toolbox::right() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },
//                         VirtualKeyCode::W => {
//                             camera.translate(-toolbox::forward() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },
//                         VirtualKeyCode::S => {
//                             camera.translate(-toolbox::back() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },
//                         VirtualKeyCode::C => {
//                             camera.translate(-toolbox::up() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },
//                         VirtualKeyCode::X => {
//                             camera.translate(-toolbox::down() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
//                         },

//                         // rotation
//                         VirtualKeyCode::Up => {
//                             camera.rotate(-toolbox::left() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }
//                         VirtualKeyCode::Down => {
//                             camera.rotate(-toolbox::right() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }
//                         VirtualKeyCode::Left => {
//                             camera.rotate(-toolbox::up() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }
//                         VirtualKeyCode::Right => {
//                             camera.rotate(-toolbox::down() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }
//                         VirtualKeyCode::Q => {
//                             camera.rotate(-toolbox::forward() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }
//                         VirtualKeyCode::E => {
//                             camera.rotate(-toolbox::back() * delta_time * ROTATION_SPEED_MULTIPLIER);
//                         }


//                         _ => { }
//                     }
//                 }
//             }

//             let mut offset = 0.0;
//             for &top_heli in &terrain_node.children {
//                 let heading = toolbox::simple_heading_animation(elapsed + offset);

//                 unsafe {
//                     (*top_heli).position = glm::vec3(heading.x, 0.0, heading.z);
//                     let mut heli = (*top_heli).children[0];

//                     (*heli).rotation = glm::vec3(heading.pitch, heading.yaw, -heading.roll);

//                     let mut main_rotor = (*heli).children[0];
//                     (*main_rotor).rotation = glm::vec3(0.0, elapsed*30.0, 0.0);

//                     let mut tail_rotor = (*heli).children[1];
//                     (*tail_rotor).rotation = glm::vec3(elapsed*30.0, 0.0, 0.0);
//                 }

//                 offset += 1.6;
//             }

//             let view_projection: glm::Mat4 = projection * camera.view();

//             // Drawing
//             unsafe {
//                 gl::ClearColor(0.163, 0.163, 0.163, 1.0);
//                 gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

//                 root.update_transformations(&glm::identity());
//                 root.draw(program_id, &view_projection);
//             }

//             context.swap_buffers().unwrap();
//         }
//     });

//     // Keep track of the health of the rendering thread
//     let render_thread_healthy = Arc::new(RwLock::new(true));
//     let render_thread_watchdog = Arc::clone(&render_thread_healthy);
//     thread::spawn(move || {
//         if !render_thread.join().is_ok() {
//             if let Ok(mut health) = render_thread_watchdog.write() {
//                 println!("Render thread panicked!");
//                 *health = false;
//             }
//         }
//     });

//     // Start the event loop -- This is where window events get handled
//     el.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Wait;

//         // Terminate program if render thread panics
//         if let Ok(health) = render_thread_healthy.read() {
//             if *health == false {
//                 *control_flow = ControlFlow::Exit;
//             }
//         }

//         match event {
//             Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
//                 *control_flow = ControlFlow::Exit;
//             },
//             // Keep track of currently pressed keys to send to the rendering thread
//             Event::WindowEvent { event: WindowEvent::KeyboardInput {
//                 input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

//                 if let Ok(mut keys) = arc_pressed_keys.lock() {
//                     match key_state {
//                         Released => {
//                             if keys.contains(&keycode) {
//                                 let i = keys.iter().position(|&k| k == keycode).unwrap();
//                                 keys.remove(i);
//                             }
//                         },
//                         Pressed => {
//                             if !keys.contains(&keycode) {
//                                 keys.push(keycode);
//                             }
//                         }
//                     }
//                 }

//                 // Handle escape separately
//                 match keycode {
//                     Escape => {
//                         *control_flow = ControlFlow::Exit;
//                     },
//                     _ => { }
//                 }
//             },
//             _ => { }
//         }
//     });
// }

// fn build_helicopter(helicopter: &mesh::Helicopter, node: &mut scene_graph::Node) {
//     let mut helicopter_node = scene_graph::SceneNode::new();

//     let main_rotor_node = scene_graph::SceneNode::from_mesh(&helicopter.main_rotor);
//     helicopter_node.add_child(&main_rotor_node);

//     let mut tail_rotor_node = scene_graph::SceneNode::from_mesh(&helicopter.tail_rotor);
//     tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
//     helicopter_node.add_child(&tail_rotor_node);

//     helicopter_node.add_child(&scene_graph::SceneNode::from_mesh(&helicopter.body));
//     helicopter_node.add_child(&scene_graph::SceneNode::from_mesh(&helicopter.door));

//     node.add_child(&helicopter_node);
// }