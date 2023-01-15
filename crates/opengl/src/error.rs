#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Image '{0}', has incompatible colour type: {1:?}")]
    InvalidImageColourType(std::path::PathBuf, image::ColorType),
    #[error("Failed to compile shader: {0}")]
    ShaderCompile(String),
    #[error("Failed to link shaders: {0}")]
    ProgramLink(String),
    #[error("Could not find uniform with name: {0}")]
    MissingUniform(String),
}
