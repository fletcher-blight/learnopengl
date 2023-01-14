extern crate anyhow;
extern crate env_logger;
extern crate gl;
extern crate glm;
extern crate image;
extern crate log;
extern crate sdl2;
extern crate thiserror;

use gl::types::*;
use sdl2::event::Event;
use std::ffi::CString;

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
    let shader_program_id = unsafe {
        let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_source = CString::new(VERTEX_SHADER)?;
        gl::ShaderSource(
            vertex_shader_id,
            1,
            &vertex_source.as_c_str().as_ptr() as _,
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader_id);
        check_compile(vertex_shader_id)?;

        let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_source = CString::new(FRAGMENT_SHADER)?;
        gl::ShaderSource(
            fragment_shader_id,
            1,
            &fragment_source.as_c_str().as_ptr() as _,
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader_id);
        check_compile(fragment_shader_id)?;

        let shader_program_id = gl::CreateProgram();
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        check_link(shader_program_id)?;

        gl::DetachShader(shader_program_id, fragment_shader_id);
        gl::DetachShader(shader_program_id, vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
        gl::DeleteShader(vertex_shader_id);

        shader_program_id
    };
    log::info!("Compiling shaders ... complete");

    log::info!("Loading Assets ...");

    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5,
        -0.5, 0.5,
        0.5, 0.5,
        0.5, -0.5f32,
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 2,
        0, 2, 3u32
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

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as _,
            std::ptr::null(),
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
            gl::UseProgram(shader_program_id);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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

const VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 330 core

out vec4 colour;

void main() {
    colour = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

fn check_compile(id: GLuint) -> anyhow::Result<()> {
    let mut success: GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success != gl::FALSE as i32 {
        return Ok(());
    }

    let mut length: GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
    }

    let error = create_space_allocated_cstring(length as usize)?;
    unsafe {
        gl::GetShaderInfoLog(
            id,
            length,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        );
    }
    anyhow::bail!(error.to_string_lossy().into_owned())
}

fn check_link(id: GLuint) -> anyhow::Result<()> {
    let mut success: GLint = 0;
    unsafe {
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }
    if success != gl::FALSE as i32 {
        return Ok(());
    }

    let mut length: GLint = 0;
    unsafe {
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut length);
    }

    let error = create_space_allocated_cstring(length as usize)?;
    unsafe {
        gl::GetProgramInfoLog(
            id,
            length,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        );
    }
    anyhow::bail!(error.to_string_lossy().into_owned())
}

fn create_space_allocated_cstring(length: usize) -> anyhow::Result<CString> {
    let space = " ".repeat(length);
    Ok(CString::new(space)?)
}
