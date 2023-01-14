extern crate anyhow;
extern crate gl;
extern crate thiserror;

use gl::types::*;
use std::ffi::CString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to compile shader: {0}")]
    Compile(String),
    #[error("Failed to link shaders: {0}")]
    Link(String),
}

pub struct Shader {
    id: GLuint,
}

pub struct ShaderProgram {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Shader {
    pub fn new(source: &str, shader_type: GLenum) -> anyhow::Result<Self> {
        let id = unsafe {
            let id = gl::CreateShader(shader_type);
            let source_str = CString::new(source)?;
            gl::ShaderSource(
                id,
                1,
                &source_str.as_c_str().as_ptr() as _,
                std::ptr::null(),
            );
            gl::CompileShader(id);
            id
        };

        let mut success: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success != gl::FALSE as i32 {
            return Ok(Shader { id });
        }

        let mut length: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
        }

        let error = CString::new(" ".repeat(length as usize))?;
        unsafe {
            gl::GetShaderInfoLog(
                id,
                length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }

        Err(anyhow::Error::new(Error::Compile(
            error.to_string_lossy().into_owned(),
        )))
    }
}

impl ShaderProgram {
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> anyhow::Result<Self> {
        let id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);
            gl::DetachShader(id, fragment_shader.id);
            gl::DetachShader(id, vertex_shader.id);
            id
        };

        let mut success: GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success != gl::FALSE as i32 {
            return Ok(ShaderProgram { id });
        }

        let mut length: GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut length);
        }

        let error = CString::new(" ".repeat(length as usize))?;
        unsafe {
            gl::GetProgramInfoLog(
                id,
                length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }

        Err(anyhow::Error::new(Error::Link(
            error.to_string_lossy().into_owned(),
        )))
    }

    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}
