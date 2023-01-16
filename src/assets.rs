extern crate anyhow;
extern crate gl;
extern crate image;
extern crate thiserror;

use crate::opengl;
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

#[derive(Clone)]
pub struct Texture {
    pub(crate) id: GLuint,
}

pub struct CubeMap<Data> {
    pub right: Data,
    pub left: Data,
    pub top: Data,
    pub bottom: Data,
    pub back: Data,
    pub front: Data,
}

pub struct IndexedMesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    num_indices: usize,
    textures: Vec<Texture>,
}

pub struct PointsMesh {
    vao: GLuint,
    vbo: GLuint,
    num_points: usize,
    textures: Vec<Texture>,
}

pub trait Vertex {
    fn attributes() -> &'static [Attribute];
}

#[derive(Copy, Clone)]
pub enum AttributeType {
    F32,
    U32,
}

pub struct Attribute {
    pub attribute_type: AttributeType,
    pub count: usize,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Drop for IndexedMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Drop for PointsMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Texture {
    pub fn from_file_2d<P>(
        shader_program: &ShaderProgram,
        texture_shader_name: &str,
        index: u32,
        texture_filename: P,
    ) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let (texture_image, texture_colour_type) =
            Texture::load_image_file(texture_filename, true)?;

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
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
        };

        let texture_location = shader_program.locate_uniform(texture_shader_name)?;
        shader_program.enable()?;
        opengl::set_uniform_i32(texture_location, index as _)?;

        Ok(Texture { id })
    }

    pub fn from_file_cubemap<P>(
        shader_program: &ShaderProgram,
        texture_shader_name: &str,
        index: u32,
        texture_filenames: CubeMap<P>,
    ) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let filenames = [
            texture_filenames.right,
            texture_filenames.left,
            texture_filenames.top,
            texture_filenames.bottom,
            texture_filenames.front,
            texture_filenames.back,
        ];

        let texture_images: Vec<_> = filenames
            .iter()
            .map(|path| Texture::load_image_file(path, false))
            .collect::<anyhow::Result<_>>()?;

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as _,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as _,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as _,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as _,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as _,
            );
        }

        for (index, (texture_image, texture_colour_type)) in texture_images.into_iter().enumerate()
        {
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32,
                    0,
                    gl::RGB as _,
                    texture_image.width() as _,
                    texture_image.height() as _,
                    0,
                    texture_colour_type,
                    gl::UNSIGNED_BYTE,
                    texture_image.as_bytes().as_ptr() as _,
                );
            }
        }

        let texture_location = shader_program.locate_uniform(texture_shader_name)?;
        shader_program.enable()?;
        opengl::set_uniform_i32(texture_location, index as _)?;

        Ok(Texture { id })
    }

    fn load_image_file<P>(
        texture_filename: P,
        flip: bool,
    ) -> anyhow::Result<(image::DynamicImage, GLenum)>
    where
        P: AsRef<Path>,
    {
        let texture_image = image::open(&texture_filename)
            .with_context(|| format!("Could not open image {:?}", texture_filename.as_ref()))?;

        let texture_image = if flip {
            texture_image.flipv()
        } else {
            texture_image
        };

        let texture_colour_type = match texture_image.color() {
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            _ => anyhow::bail!(Error::InvalidImageColourType(
                texture_filename.as_ref().into(),
                texture_image.color()
            )),
        };

        Ok((texture_image, texture_colour_type))
    }
}

impl IndexedMesh {
    pub fn new<Vertex>(
        indices: &[u32],
        vertices: &[Vertex],
        textures: Vec<Texture>,
    ) -> anyhow::Result<Self>
    where
        Vertex: crate::Vertex,
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
                (vertices.len() * std::mem::size_of::<Vertex>()) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as _,
                indices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            let attributes = Vertex::attributes();

            let stride: usize = attributes
                .iter()
                .map(|attribute| attribute.count * attribute.attribute_type.num_bytes())
                .sum();

            let mut offset = 0;
            for (index, attribute) in attributes.iter().enumerate() {
                gl::EnableVertexAttribArray(index as _);
                gl::VertexAttribPointer(
                    index as _,
                    attribute.count as _,
                    attribute.attribute_type.into(),
                    gl::FALSE,
                    stride as _,
                    offset as _,
                );

                offset += attribute.count * attribute.attribute_type.num_bytes();
            }

            gl::BindVertexArray(0);
        };

        Ok(IndexedMesh {
            vao,
            vbo,
            ebo,
            num_indices,
            textures,
        })
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            for (index, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices as _,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

impl PointsMesh {
    pub fn new<Vertex>(vertices: &[Vertex], textures: Vec<Texture>) -> anyhow::Result<Self>
    where
        Vertex: crate::Vertex,
    {
        let attributes = Vertex::attributes();
        let stride: usize = attributes
            .iter()
            .map(|attribute| attribute.count * attribute.attribute_type.num_bytes())
            .sum();

        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as _,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            let mut offset = 0;
            for (index, attribute) in attributes.iter().enumerate() {
                gl::EnableVertexAttribArray(index as _);
                gl::VertexAttribPointer(
                    index as _,
                    attribute.count as _,
                    attribute.attribute_type.into(),
                    gl::FALSE,
                    stride as _,
                    offset as _,
                );

                offset += attribute.count * attribute.attribute_type.num_bytes();
            }

            gl::BindVertexArray(0);
        };

        let num_points = vertices.len()
            * attributes
                .iter()
                .map(|attribute| attribute.count)
                .sum::<usize>();

        Ok(PointsMesh {
            vao,
            vbo,
            num_points,
            textures,
        })
    }

    pub fn draw(&self) {
        unsafe {
            for (index, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture.id);
            }

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_points as _);
        }
    }
}

impl AttributeType {
    fn num_bytes(&self) -> usize {
        match self {
            AttributeType::F32 => std::mem::size_of::<f32>(),
            AttributeType::U32 => std::mem::size_of::<u32>(),
        }
    }
}

impl From<AttributeType> for GLenum {
    fn from(value: AttributeType) -> Self {
        match value {
            AttributeType::F32 => gl::FLOAT,
            AttributeType::U32 => gl::UNSIGNED_INT,
        }
    }
}
