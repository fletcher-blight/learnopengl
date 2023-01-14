extern crate anyhow;
extern crate gl;
extern crate image;
extern crate thiserror;

use crate::shader::ShaderProgram;
use anyhow::Context;
use gl::types::*;
use image::ColorType;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Image '{0}', has incompatible colour type: {1:?}")]
    InvalidImageColourType(std::path::PathBuf, ColorType),
}

#[repr(C, packed)]
pub struct VertexND<const N: usize> {
    pub position: [f32; N],
    pub tex_coords: [f32; 2],
}

pub type Vertex2D = VertexND<2>;
pub type Vertex3D = VertexND<3>;

#[derive(Clone)]
pub struct Texture {
    id: GLuint,
}

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    _textures: Vec<Texture>,
    num_indices: usize,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Texture {
    pub fn new<P>(
        shader_program: &ShaderProgram,
        texture_shader_name: &str,
        index: u32,
        texture_filename: P,
    ) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let texture_image = image::open(&texture_filename)
            .with_context(|| format!("Could not open image {:?}", texture_filename.as_ref()))?
            .flipv();
        let texture_colour_type = match texture_image.color() {
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            _ => anyhow::bail!(Error::InvalidImageColourType(
                texture_filename.as_ref().into(),
                texture_image.color()
            )),
        };

        shader_program.enable();
        let texture_location = shader_program.locate_uniform(texture_shader_name)?;

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                texture_image.width() as _,
                texture_image.height() as _,
                0,
                texture_colour_type,
                gl::UNSIGNED_BYTE,
                texture_image.as_bytes().as_ptr() as _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::Uniform1i(texture_location, index as _);
        };

        Ok(Texture { id })
    }
}

impl Mesh {
    pub fn new<P, const N: usize>(
        shader_program: &ShaderProgram,
        vertices: &[VertexND<N>],
        indices: &[u32],
        textures: &[(P, &str)],
    ) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        let num_indices = indices.len();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<VertexND<N>>()) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as _,
                indices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            let stride = (N + 2) * std::mem::size_of::<f32>();

            shader_program.enable();
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                N as _,
                gl::FLOAT,
                gl::FALSE,
                stride as _,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride as _,
                (N * std::mem::size_of::<f32>()) as _,
            );

            gl::BindVertexArray(0);
        };

        let textures = textures
            .iter()
            .enumerate()
            .map(|(index, (path, name))| Texture::new(&shader_program, name, index as _, path))
            .collect::<anyhow::Result<_>>()?;

        Ok(Mesh {
            vao,
            vbo,
            ebo,
            _textures: textures,
            num_indices,
        })
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices as _,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
