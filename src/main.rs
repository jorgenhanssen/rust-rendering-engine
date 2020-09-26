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
// mod renderable;
mod camera;
mod mesh;

use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

const TRANSLATION_SPEED_MULTIPLIER: f32 = 30.0;
const ROTATION_SPEED_MULTIPLIER: f32 = 1.0;


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
        let mvp_id: gl::types::GLint;
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

            // Set up MVP
            let mvp_name = "mvp";
            mvp_id = gl::GetUniformLocation(program_id, std::ffi::CString::new(mvp_name).unwrap().as_ptr());
        }
    
        let projection: glm::Mat4 = glm::perspective(SCREEN_W  as f32 / SCREEN_H as f32, 1.0, 1.0, 1000.0);
        let mut camera = camera::Camera::new();

        camera.set_position(glm::vec3(0.0, 0.0, -2.0));

        // Create the object that should be rendered
        // let shape = renderable::circle(128);
        // let shape = renderable::triangle();
        // let shape = renderable::square();
        // let shape = renderable::sine(128, 3.0, 0.01);
        // let shape = renderable::from_obj("resources/teapot.obj");

        // Assignment-specific:
        // let shape = renderable::task_1_b();
        // let shape = renderable::task_2_a();

        let helicopter = mesh::Helicopter::load("resources/helicopter.obj");
        let objects: Vec<mesh::Mesh> = vec![
            mesh::Terrain::load("resources/lunarsurface.obj"),
            helicopter.body,
            helicopter.door,
            helicopter.main_rotor,
            helicopter.tail_rotor,
        ];

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
                            camera.translate(-camera::left() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::D => {
                            camera.translate(-camera::right() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::W => {
                            camera.translate(-camera::forward() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::S => {
                            camera.translate(-camera::back() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::C => {
                            camera.translate(-camera::up() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },
                        VirtualKeyCode::X => {
                            camera.translate(-camera::down() * delta_time * TRANSLATION_SPEED_MULTIPLIER);
                        },

                        // rotation
                        VirtualKeyCode::Up => {
                            camera.rotate(-camera::left() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Down => {
                            camera.rotate(-camera::right() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Left => {
                            camera.rotate(-camera::up() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Right => {
                            camera.rotate(-camera::down() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::Q => {
                            camera.rotate(-camera::forward() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }
                        VirtualKeyCode::E => {
                            camera.rotate(-camera::back() * delta_time * ROTATION_SPEED_MULTIPLIER);
                        }


                        _ => { }
                    }
                }
            }

            // Set mvp
            unsafe {
                let mvp: glm::Mat4 = projection * camera.view();
                gl::UniformMatrix4fv(mvp_id, 1, gl::FALSE, mvp.as_ptr() as *const _);
            }

            // Drawing
            unsafe {
                gl::ClearColor(0.163, 0.163, 0.163, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                for object in objects.iter() {
                    object.draw();
                }
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