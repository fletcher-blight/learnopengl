use crate::error::*;
use crate::shader::*;
use anyhow::Context;
use image::DynamicImage;
use opengl_sys::DataType;
use std::path::Path;

pub trait TextureType {
    fn bind(&self) -> anyhow::Result<()>;
}

pub struct ShaderProgramTexture<'a, Texture: TextureType> {
    texture: &'a Texture,
    index: u32,
}

impl<'a, Texture: TextureType> ShaderProgramTexture<'a, Texture> {
    pub fn new(
        texture: &'a Texture,
        shader_program: &ShaderProgram,
        name: &str,
        index: u32,
    ) -> anyhow::Result<Self> {
        shader_program.enable()?;
        let location = shader_program.locate_uniform(name)?;
        opengl_sys::set_uniform_i32(location, index as _)?;
        Ok(Self { texture, index })
    }

    pub fn draw(&self) -> anyhow::Result<()> {
        opengl_sys::active_texture(self.index)?;
        self.texture.bind()?;
        Ok(())
    }
}

pub struct TextureImage2D {
    id: opengl_sys::TextureID,
}

impl TextureImage2D {
    pub fn load_from_file<P>(texture_filename: &P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let (image, format) = load_image(texture_filename, true)?;
        Self::load_from_memory(image.as_bytes(), format, image.width(), image.height())
    }

    pub fn load_from_memory(
        data: &[u8],
        format: opengl_sys::TextureFormat,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let texture = Self {
            id: opengl_sys::create_texture(),
        };

        opengl_sys::bind_texture(texture.id, opengl_sys::TextureTarget::Image2D)?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::WrapS,
            opengl_sys::TextureParameterValue::Repeat,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::WrapT,
            opengl_sys::TextureParameterValue::Repeat,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::MinFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::MagFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;
        opengl_sys::load_texture_image2d(
            opengl_sys::TextureTarget::Image2D,
            0,
            opengl_sys::TextureFormat::RGB,
            width as _,
            height as _,
            format,
            DataType::U8,
            data,
        )?;
        opengl_sys::generate_mipmaps(opengl_sys::TextureTarget::Image2D)?;

        Ok(texture)
    }
}

impl TextureType for TextureImage2D {
    fn bind(&self) -> anyhow::Result<()> {
        opengl_sys::bind_texture(self.id, opengl_sys::TextureTarget::Image2D)?;
        Ok(())
    }
}

impl Drop for TextureImage2D {
    fn drop(&mut self) {
        opengl_sys::delete_texture(self.id).expect("Failed to delete texture 2D image");
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
    fn get_face(&self, face_target: opengl_sys::TextureCubeMapFaceTarget) -> &D {
        match face_target {
            opengl_sys::TextureCubeMapFaceTarget::Right => &self.right,
            opengl_sys::TextureCubeMapFaceTarget::Left => &self.left,
            opengl_sys::TextureCubeMapFaceTarget::Top => &self.top,
            opengl_sys::TextureCubeMapFaceTarget::Bottom => &self.bottom,
            opengl_sys::TextureCubeMapFaceTarget::Back => &self.back,
            opengl_sys::TextureCubeMapFaceTarget::Front => &self.front,
        }
    }
}

pub struct TextureCubeMap {
    id: opengl_sys::TextureID,
}

impl TextureCubeMap {
    pub fn load_from_file<P>(texture_filenames: &CubeMap<P>) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let texture = Self {
            id: opengl_sys::create_texture(),
        };

        opengl_sys::bind_texture(texture.id, opengl_sys::TextureTarget::CubeMap)?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::CubeMap,
            opengl_sys::TextureParameterName::WrapS,
            opengl_sys::TextureParameterValue::ClampToEdge,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::CubeMap,
            opengl_sys::TextureParameterName::WrapT,
            opengl_sys::TextureParameterValue::ClampToEdge,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::CubeMap,
            opengl_sys::TextureParameterName::WrapR,
            opengl_sys::TextureParameterValue::ClampToEdge,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::CubeMap,
            opengl_sys::TextureParameterName::MinFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::CubeMap,
            opengl_sys::TextureParameterName::MagFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;

        for target_face in [
            opengl_sys::TextureCubeMapFaceTarget::Right,
            opengl_sys::TextureCubeMapFaceTarget::Left,
            opengl_sys::TextureCubeMapFaceTarget::Top,
            opengl_sys::TextureCubeMapFaceTarget::Bottom,
            opengl_sys::TextureCubeMapFaceTarget::Back,
            opengl_sys::TextureCubeMapFaceTarget::Front,
        ] {
            let texture_filename = texture_filenames.get_face(target_face);
            let (image, format) = load_image(texture_filename, false)?;

            opengl_sys::load_texture_image2d(
                opengl_sys::TextureTarget::CubeMapFace(target_face),
                0,
                opengl_sys::TextureFormat::RGB,
                image.width() as _,
                image.height() as _,
                format,
                DataType::U8,
                image.as_bytes(),
            )?;
        }

        Ok(texture)
    }
}

impl TextureType for TextureCubeMap {
    fn bind(&self) -> anyhow::Result<()> {
        opengl_sys::bind_texture(self.id, opengl_sys::TextureTarget::CubeMap)?;
        Ok(())
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        opengl_sys::delete_texture(self.id).expect("Failed to delete texture cubemap");
    }
}

fn load_image<P>(
    filename: &P,
    flip_verticals: bool,
) -> anyhow::Result<(DynamicImage, opengl_sys::TextureFormat)>
where
    P: AsRef<Path>,
{
    let image = image::open(filename)
        .with_context(|| format!("Could not open image {:?}", filename.as_ref()))?;

    let image = if flip_verticals { image.flipv() } else { image };

    let format = match image.color() {
        image::ColorType::Rgb8 => Ok(opengl_sys::TextureFormat::RGB),
        image::ColorType::Rgba8 => Ok(opengl_sys::TextureFormat::RGBA),
        _ => Err(Error::InvalidImageColourType(
            filename.as_ref().into(),
            image.color(),
        )),
    }?;

    Ok((image, format))
}
