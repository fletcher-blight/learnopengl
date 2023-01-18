pub use opengl_sys::ShaderType;

use crate::error::*;

pub use opengl_sys::UniformLocation;

pub struct Shader {
    id: opengl_sys::ShaderID,
}

pub struct ShaderProgram {
    id: opengl_sys::ProgramID,
}

impl Drop for Shader {
    fn drop(&mut self) {
        opengl_sys::delete_shader(self.id).expect("Failed to Delete Shader");
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        opengl_sys::delete_program(self.id).expect("Failed to Delete Shader Program");
    }
}

impl Shader {
    pub fn new(source: &str, shader_type: ShaderType) -> anyhow::Result<Self> {
        let shader = Shader {
            id: opengl_sys::create_shader(shader_type),
        };
        opengl_sys::set_shader_source(shader.id, source)?;
        opengl_sys::compile_shader(shader.id)?;
        if opengl_sys::get_shader_parameter(shader.id, opengl_sys::ShaderParameter::CompileStatus)?
            != 0
        {
            Ok(shader)
        } else {
            let buffer_size = opengl_sys::get_shader_parameter(
                shader.id,
                opengl_sys::ShaderParameter::InfoLogLength,
            )?;
            Err(anyhow::Error::new(Error::ShaderCompile(
                opengl_sys::get_shader_info_log(shader.id, buffer_size)?,
            )))
        }
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> anyhow::Result<Self> {
        let shader_program = ShaderProgram {
            id: opengl_sys::create_program(),
        };

        for shader in shaders {
            opengl_sys::attach_shader(shader_program.id, shader.id)?;
        }

        opengl_sys::link_program(shader_program.id)?;

        for shader in shaders {
            opengl_sys::detach_shader(shader_program.id, shader.id)?;
        }

        if opengl_sys::get_program_paramter(
            shader_program.id,
            opengl_sys::ProgramParameter::LinkStatus,
        )? != 0
        {
            Ok(shader_program)
        } else {
            let buffer_size = opengl_sys::get_program_paramter(
                shader_program.id,
                opengl_sys::ProgramParameter::InfoLogLength,
            )?;
            Err(anyhow::Error::new(Error::ProgramLink(
                opengl_sys::get_program_info_log(shader_program.id, buffer_size)?,
            )))
        }
    }

    pub fn enable(&self) -> anyhow::Result<()> {
        opengl_sys::use_program(self.id)?;
        Ok(())
    }

    pub fn locate_uniform(&self, name: &str) -> anyhow::Result<UniformLocation> {
        let location = opengl_sys::get_uniform_location(self.id, name)?;
        Ok(location.ok_or_else(|| Error::MissingUniform(name.into()))?)
    }
}
