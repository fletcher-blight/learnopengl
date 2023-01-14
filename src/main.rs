mod assets;
mod camera;
mod cube;
mod shader;
mod window;

extern crate anyhow;
extern crate env_logger;
extern crate log;
extern crate nalgebra_glm;
extern crate rand;

use assets::*;
use camera::Camera;
use gl::types::GLint;
use nalgebra_glm as glm;
use rand::Rng;
use shader::*;
use window::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut rng = rand::thread_rng();

    log::info!("Window initialisation ...");
    let window = Window::new("Rust Renderer", 1920, 1080)?;
    log::info!("Window initialisation ... complete");

    log::info!("Compiling shaders ...");
    let shader_program = ShaderProgram::new(
        &Shader::new(include_str!("../assets/shader.vert"), gl::VERTEX_SHADER)?,
        &Shader::new(include_str!("../assets/shader.frag"), gl::FRAGMENT_SHADER)?,
    )?;
    log::info!("Compiling shaders ... complete");

    log::info!("Loading assets ...");
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
    log::info!("Loading assets ... complete");

    log::info!("Initialising game logic ...");

    let model_location = shader_program.locate_uniform("model")?;
    let view_location = shader_program.locate_uniform("view")?;
    let projection_location = shader_program.locate_uniform("projection")?;

    let camera = Camera::new();

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

    log::info!("Initialising game logic ... complete");

    log::info!("Game loop");
    window.run(|passed_time, window_size, _events| {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_program.enable();

        set_uniform_mat4(view_location, &camera.calculate_view());
        set_uniform_mat4(
            projection_location,
            &camera.calculate_projection(window_size),
        );

        for cube_position in &cube_positions {
            set_uniform_mat4(model_location, &calculate_model(passed_time, cube_position));
            mesh.draw();
        }
    })
}

fn set_uniform_mat4(uniform_location: GLint, mat: &glm::Mat4) {
    unsafe { gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, glm::value_ptr(mat).as_ptr()) }
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
