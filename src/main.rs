mod assets;
mod shader;

extern crate anyhow;
extern crate env_logger;
extern crate gl;
extern crate glm;
extern crate log;
extern crate sdl2;
extern crate thiserror;

use assets::*;
use sdl2::event::Event;
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
        &[
            Vertex2D {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                position: [-0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex2D {
                position: [0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                position: [0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
        ],
        &[0, 1, 2, 0, 2, 3],
        &[
            (
                std::env::current_dir()?.join("assets/splatoon-face.jpeg"),
                "tex1",
            ),
            (std::env::current_dir()?.join("assets/ship_C.png"), "tex2"),
        ],
    )?;
    log::info!("Loading Assets ... complete");

    log::info!("Game Loop");
    let mut event_pump = sdl.event_pump().map_err(Error::SDL)?;
    'main: loop {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.enable();
            mesh.draw();
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }
    }

    log::info!("Goodbye");
    Ok(())
}
