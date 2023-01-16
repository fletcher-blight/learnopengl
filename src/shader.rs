extern crate anyhow;
extern crate thiserror;

use crate::opengl;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to compile shader: {0}")]
    Compile(String),
    #[error("Failed to link shaders: {0}")]
    Link(String),
    #[error("Could not find uniform with name: {0}")]
    MissingUniform(String),
}

pub use opengl::ShaderType;

pub struct Shader {
    id: opengl::ShaderID,
}

pub struct ShaderProgram {
    id: opengl::ProgramID,
}

impl Drop for Shader {
    fn drop(&mut self) {
        opengl::delete_shader(self.id).expect("Failed to Delete Shader");
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        opengl::delete_program(self.id).expect("Failed to Delete Shader Program");
    }
}

impl Shader {
    pub fn new(source: &str, shader_type: ShaderType) -> anyhow::Result<Self> {
        let shader = Shader {
            id: opengl::create_shader(shader_type),
        };
        opengl::set_shader_source(shader.id, source)?;
        opengl::compile_shader(shader.id)?;
        if opengl::get_shader_parameter(shader.id, opengl::ShaderParameter::CompileStatus)? != 0 {
            Ok(shader)
        } else {
            let buffer_size =
                opengl::get_shader_parameter(shader.id, opengl::ShaderParameter::InfoLogLength)?;
            Err(anyhow::Error::new(Error::Compile(
                opengl::get_shader_info_log(shader.id, buffer_size)?,
            )))
        }
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> anyhow::Result<Self> {
        let shader_program = ShaderProgram {
            id: opengl::create_program(),
        };

        for shader in shaders {
            opengl::attach_shader(shader_program.id, shader.id)?;
        }

        opengl::link_program(shader_program.id)?;

        for shader in shaders {
            opengl::detach_shader(shader_program.id, shader.id)?;
        }

        if opengl::get_program_paramter(shader_program.id, opengl::ProgramParameter::LinkStatus)?
            != 0
        {
            Ok(shader_program)
        } else {
            let buffer_size = opengl::get_program_paramter(
                shader_program.id,
                opengl::ProgramParameter::InfoLogLength,
            )?;
            Err(anyhow::Error::new(Error::Link(
                opengl::get_program_info_log(shader_program.id, buffer_size)?,
            )))
        }
    }

    pub fn enable(&self) -> anyhow::Result<()> {
        opengl::use_program(self.id)?;
        Ok(())
    }

    pub fn locate_uniform(&self, name: &str) -> anyhow::Result<opengl::UniformLocation> {
        let location = opengl::get_uniform_location(self.id, name)?;
        Ok(location.ok_or(Error::MissingUniform(name.into()))?)
    }
}
