mod assets;
mod cube;
mod shader;

extern crate anyhow;
extern crate env_logger;
extern crate gl;
extern crate log;
extern crate nalgebra_glm;
extern crate rand;
extern crate sdl2;
extern crate thiserror;

use assets::*;
use nalgebra_glm as glm;
use rand::Rng;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use shader::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SDL Failure: {0}")]
    SDL(String),
    #[error("Failed to load image: {0}")]
    Image(String),
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut rng = rand::thread_rng();
    log::info!("Window initialisation ...");

    let sdl = sdl2::init().map_err(Error::SDL)?;
    let sdl_video = sdl.video().map_err(Error::SDL)?;
    {
        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = sdl_video
        .window("Rust Renderer", 1920, 1200)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context();
    gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe { gl::Enable(gl::DEPTH_TEST) };

    log::info!("Window initialisation ... complete");

    log::info!("Compiling shaders ...");
    let shader_program = ShaderProgram::new(
        &Shader::new(include_str!("../assets/shader.vert"), gl::VERTEX_SHADER)?,
        &Shader::new(include_str!("../assets/shader.frag"), gl::FRAGMENT_SHADER)?,
    )?;
    log::info!("Compiling shaders ... complete");

    log::info!("Loading Assets ...");
    let mesh = Mesh::new(
        &shader_program,
        &cube::VERTICES,
        &cube::INDICES,
        &[
            (
                std::env::current_dir()?.join("assets/splatoon-face.jpeg"),
                "tex1",
            ),
            (std::env::current_dir()?.join("assets/ship_C.png"), "tex2"),
        ],
    )?;

    let (window_width, window_height) = window.size();
    let aspect_ratio = window_width as f32 / window_height as f32;
    let projection: glm::Mat4 = glm::perspective(aspect_ratio, 45.0f32.to_radians(), 0.1, 100.0);

    let model_location = shader_program.locate_uniform("model")?;
    let view_location = shader_program.locate_uniform("view")?;
    let projection_location = shader_program.locate_uniform("projection")?;

    let set_mat4 = |mat: &glm::Mat4, loc| unsafe {
        gl::UniformMatrix4fv(loc, 1, gl::FALSE, glm::value_ptr(mat).as_ptr())
    };

    set_mat4(&glm::one(), view_location);
    set_mat4(&projection, projection_location);

    let cube_positions: Vec<_> = std::iter::repeat_with(|| {
        glm::vec3(
            rng.gen_range(-3.0..3.0),
            rng.gen_range(-3.0..3.0),
            rng.gen_range(-5.0..5.0),
        )
    })
    .filter(|pos| *pos != glm::vec3(0.0, 0.0, 0.0))
    .take(10)
    .collect();

    log::info!("Loading Assets ... complete");

    log::info!("Game Loop");
    let start_time = std::time::Instant::now();
    let mut event_pump = sdl.event_pump().map_err(Error::SDL)?;
    'main: loop {
        let passed_time = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs_f32();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_program.enable();
        for cube_position in &cube_positions {
            let model = calculate_model(passed_time, cube_position);
            set_mat4(&model, model_location);
            mesh.draw();
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                _ => {}
            }
        }
    }

    log::info!("Goodbye");
    Ok(())
}

fn calculate_model(passed_time: f32, cube_position: &glm::Vec3) -> glm::Mat4 {
    glm::rotate(
        &glm::translate(
            &glm::rotate(
                &glm::one(),
                (passed_time * 30.0).to_radians(),
                &glm::vec3(1.0, 1.0, 1.0),
            ),
            cube_position,
        ),
        (passed_time * 50.0).to_radians(),
        cube_position,
    )
}
