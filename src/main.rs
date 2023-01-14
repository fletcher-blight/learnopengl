mod shader;

extern crate anyhow;
extern crate env_logger;
extern crate gl;
extern crate glm;
extern crate image;
extern crate log;
extern crate sdl2;
extern crate thiserror;

use sdl2::event::Event;
use shader::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SDL Failure: {0}")]
    SDL(String),
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
    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5, 1.0, 0.0, 0.0,
        0.0, 0.5, 0.0, 1.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0f32,
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 2,
    ];

    let vao = unsafe {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as _,
            indices.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        shader_program.enable();
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as _,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as _,
            (2 * std::mem::size_of::<f32>()) as _,
        );

        gl::BindVertexArray(0);
        vao
    };
    log::info!("Loading Assets ... complete");

    log::info!("Game Loop");
    let mut event_pump = sdl.event_pump().map_err(Error::SDL)?;
    'main: loop {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.enable();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
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
