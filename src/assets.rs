extern crate anyhow;
extern crate image;
extern crate thiserror;

use crate::opengl;
use crate::shader::ShaderProgram;
use anyhow::Context;
use image::{ColorType, DynamicImage};
use std::path::Path;

pub use opengl::BufferTarget;
pub use opengl::DrawMode;
pub use opengl::VertexAttributeSize as BufferAttributeSize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Image '{0}', has incompatible colour type: {1:?}")]
    InvalidImageColourType(std::path::PathBuf, ColorType),
}

#[derive(Clone)]
pub struct VertexArray {
    id: opengl::VertexArrayID,
}

impl VertexArray {
    pub fn new() -> Self {
        Self {
            id: opengl::create_vertex_array(),
        }
    }

    pub fn bind(&self) -> anyhow::Result<()> {
        opengl::bind_vertex_array(self.id)?;
        Ok(())
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        opengl::delete_vertex_array(self.id).expect("Failed to delete vertex array");
    }
}

#[derive(Clone)]
pub struct Buffer {
    id: opengl::BufferID,
    target: BufferTarget,
    size: u64,
}

#[derive(Copy, Clone)]
pub struct BufferAttribute {
    pub size: BufferAttributeSize,
    pub data_type: opengl::DataType,
}

pub trait BufferVertex {
    fn attribute_layout() -> &'static [BufferAttribute];
}

impl BufferVertex for u32 {
    fn attribute_layout() -> &'static [BufferAttribute] {
        &[BufferAttribute {
            size: opengl::VertexAttributeSize::Single,
            data_type: opengl::DataType::U32,
        }]
    }
}

impl Buffer {
    pub fn new(target: BufferTarget) -> Self {
        Self {
            id: opengl::create_buffer(),
            target,
            size: 0,
        }
    }

    pub fn bind<Vertex: BufferVertex>(&mut self, vertices: &[Vertex]) -> anyhow::Result<()> {
        opengl::bind_buffer(self.id, self.target)?;
        opengl::set_buffer_data(self.target, opengl::BufferUsage::StaticDraw, vertices)?;

        let attributes = Vertex::attribute_layout();
        let stride = attributes
            .iter()
            .map(|attribute| attribute.size.as_value() * attribute.data_type.num_bytes())
            .sum();

        let mut offset = 0;
        for (index, attribute) in attributes.iter().enumerate() {
            opengl::enable_vertex_attribute_array(index as _)?;
            opengl::vertex_attribute_pointer(
                index as _,
                attribute.size,
                attribute.data_type,
                false,
                stride,
                offset,
            )?;

            offset += attribute.size.as_value() * attribute.data_type.num_bytes();
        }

        self.size = vertices.len() as _;
        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        opengl::delete_buffer(self.id).expect("Failed to delete buffer");
    }
}

fn load_image<P>(
    filename: &P,
    flip_verticals: bool,
) -> anyhow::Result<(DynamicImage, opengl::TextureFormat)>
where
    P: AsRef<Path>,
{
    let image = image::open(filename)
        .with_context(|| format!("Could not open image {:?}", filename.as_ref()))?;

    let image = if flip_verticals { image.flipv() } else { image };

    let format = match image.color() {
        ColorType::Rgb8 => Ok(opengl::TextureFormat::RGB),
        ColorType::Rgba8 => Ok(opengl::TextureFormat::RGB),
        _ => Err(Error::InvalidImageColourType(
            filename.as_ref().into(),
            image.color(),
        )),
    }?;

    Ok((image, format))
}

pub trait TextureType {
    fn bind(&self) -> anyhow::Result<()>;
}

pub struct TextureActivationProxy<'a, Texture: TextureType> {
    texture: &'a Texture,
    index: u32,
}

impl<'a, Texture: TextureType> TextureActivationProxy<'a, Texture> {
    pub fn new(
        texture: &'a Texture,
        shader_program: &ShaderProgram,
        name: &str,
        index: u32,
    ) -> anyhow::Result<Self> {
        shader_program.enable()?;
        let location = shader_program.locate_uniform(name)?;
        opengl::set_uniform_i32(location, index as _)?;
        Ok(Self { texture, index })
    }

    pub fn activate(&self) -> anyhow::Result<()> {
        opengl::active_texture(self.index)?;
        self.texture.bind()?;
        Ok(())
    }
}

pub struct TextureImage2D {
    id: opengl::TextureID,
}

impl TextureImage2D {
    pub fn load_from_file<P>(texture_filename: &P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let (image, format) = load_image(texture_filename, true)?;
        let texture = Self {
            id: opengl::create_texture(),
        };

        opengl::bind_texture(texture.id, opengl::TextureTarget::Image2D)?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::Image2D,
            opengl::TextureParameterName::WrapS,
            opengl::TextureParameterValue::Repeat,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::Image2D,
            opengl::TextureParameterName::WrapT,
            opengl::TextureParameterValue::Repeat,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::Image2D,
            opengl::TextureParameterName::MinFilter,
            opengl::TextureParameterValue::Linear,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::Image2D,
            opengl::TextureParameterName::MagFilter,
            opengl::TextureParameterValue::Linear,
        )?;
        opengl::load_texture_image2d(
            opengl::TextureTarget::Image2D,
            0,
            opengl::TextureFormat::RGBA,
            image.width() as _,
            image.height() as _,
            format,
            opengl::DataType::U8,
            image.as_bytes(),
        )?;
        opengl::generate_mipmaps(opengl::TextureTarget::Image2D)?;

        Ok(texture)
    }
}

impl TextureType for TextureImage2D {
    fn bind(&self) -> anyhow::Result<()> {
        opengl::bind_texture(self.id, opengl::TextureTarget::Image2D)?;
        Ok(())
    }
}

impl Drop for TextureImage2D {
    fn drop(&mut self) {
        opengl::delete_texture(self.id).expect("Failed to delete texture 2D image");
    }
}

pub struct CubeMap<Data> {
    pub right: Data,
    pub left: Data,
    pub top: Data,
    pub bottom: Data,
    pub back: Data,
    pub front: Data,
}

impl<D> CubeMap<D> {
    fn get_face(&self, face_target: opengl::TextureCubeMapFaceTarget) -> &D {
        match face_target {
            opengl::TextureCubeMapFaceTarget::Right => &self.right,
            opengl::TextureCubeMapFaceTarget::Left => &self.left,
            opengl::TextureCubeMapFaceTarget::Top => &self.top,
            opengl::TextureCubeMapFaceTarget::Bottom => &self.bottom,
            opengl::TextureCubeMapFaceTarget::Back => &self.back,
            opengl::TextureCubeMapFaceTarget::Front => &self.front,
        }
    }
}

pub struct TextureCubeMap {
    id: opengl::TextureID,
}

impl TextureCubeMap {
    pub fn load_from_file<P>(texture_filenames: &CubeMap<P>) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let texture = Self {
            id: opengl::create_texture(),
        };

        opengl::bind_texture(texture.id, opengl::TextureTarget::CubeMap)?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::CubeMap,
            opengl::TextureParameterName::WrapS,
            opengl::TextureParameterValue::ClampToEdge,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::CubeMap,
            opengl::TextureParameterName::WrapT,
            opengl::TextureParameterValue::ClampToEdge,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::CubeMap,
            opengl::TextureParameterName::WrapR,
            opengl::TextureParameterValue::ClampToEdge,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::CubeMap,
            opengl::TextureParameterName::MinFilter,
            opengl::TextureParameterValue::Linear,
        )?;
        opengl::set_texture_parameter_value(
            opengl::TextureTarget::CubeMap,
            opengl::TextureParameterName::MagFilter,
            opengl::TextureParameterValue::Linear,
        )?;

        for target_face in [
            opengl::TextureCubeMapFaceTarget::Right,
            opengl::TextureCubeMapFaceTarget::Left,
            opengl::TextureCubeMapFaceTarget::Top,
            opengl::TextureCubeMapFaceTarget::Bottom,
            opengl::TextureCubeMapFaceTarget::Back,
            opengl::TextureCubeMapFaceTarget::Front,
        ] {
            let texture_filename = texture_filenames.get_face(target_face);
            let (image, format) = load_image(texture_filename, false)?;

            opengl::load_texture_image2d(
                opengl::TextureTarget::CubeMapFace(target_face),
                0,
                opengl::TextureFormat::RGB,
                image.width() as _,
                image.height() as _,
                format,
                opengl::DataType::U8,
                image.as_bytes(),
            )?;
        }

        Ok(texture)
    }
}

impl TextureType for TextureCubeMap {
    fn bind(&self) -> anyhow::Result<()> {
        opengl::bind_texture(self.id, opengl::TextureTarget::CubeMap)?;
        Ok(())
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        opengl::delete_texture(self.id).expect("Failed to delete texture cubemap");
    }
}

#[derive(Clone)]
pub struct Mesh {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Option<Buffer>,
    draw_mode: DrawMode,
}

impl Mesh {
    pub fn new(vao: VertexArray, vbo: Buffer, ebo: Option<Buffer>, draw_mode: DrawMode) -> Self {
        Self {
            vao,
            vbo,
            ebo,
            draw_mode,
        }
    }

    pub fn create_and_bind<Vertex: BufferVertex>(
        buffer_data: &[Vertex],
        indices: Option<&[u32]>,
        draw_mode: DrawMode,
    ) -> anyhow::Result<Self> {
        let vao = VertexArray::new();
        let mut vbo = Buffer::new(BufferTarget::Array);

        vao.bind()?;
        vbo.bind(buffer_data)?;
        let ebo = if let Some(indices) = indices {
            let mut buffer = Buffer::new(BufferTarget::ElementArray);
            buffer.bind(indices)?;
            Some(buffer)
        } else {
            None
        };

        opengl::bind_vertex_array(0)?;
        Ok(Self {
            vao,
            vbo,
            ebo,
            draw_mode,
        })
    }

    pub fn draw(&self) -> anyhow::Result<()> {
        self.vao.bind()?;

        if let Some(ebo) = &self.ebo {
            opengl::draw_elements(self.draw_mode, ebo.size, opengl::DataType::U32)?;
        } else {
            opengl::draw_arrays(self.draw_mode, 0, self.vbo.size)?;
        }

        Ok(())
    }
}
