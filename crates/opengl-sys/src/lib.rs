use gl::types::*;
use std::ffi::CString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown error code : {0}")]
    Unknown(GLenum),
    #[error("Invalid enum")]
    InvalidEnum,
    #[error("Invalid value")]
    InvalidValue,
    #[error("Invalid Operation")]
    InvalidOperation,
    #[error("Stack Overflow")]
    StackOverflow,
    #[error("Stack Underflow")]
    StackUnderflow,
    #[error("Stack Out of Memory")]
    StackOutOfMemory,
    #[error("Invalid Frame Buffer Operation")]
    InvalidFrameBufferOperation,
}

pub fn get_error() -> Option<Error> {
    let error_code = unsafe { gl::GetError() };
    match error_code {
        gl::NO_ERROR => None,
        gl::INVALID_ENUM => Some(Error::InvalidEnum),
        gl::INVALID_VALUE => Some(Error::InvalidValue),
        gl::INVALID_OPERATION => Some(Error::InvalidOperation),
        gl::STACK_OVERFLOW => Some(Error::StackOverflow),
        gl::STACK_UNDERFLOW => Some(Error::StackUnderflow),
        gl::OUT_OF_MEMORY => Some(Error::StackOutOfMemory),
        gl::INVALID_FRAMEBUFFER_OPERATION => Some(Error::InvalidFrameBufferOperation),
        _ => Some(Error::Unknown(error_code)),
    }
}

pub fn assert_no_error() -> Result<(), Error> {
    match get_error() {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F32,
}

impl DataType {
    pub fn num_bytes(&self) -> u32 {
        match self {
            DataType::I8 | DataType::U8 => 1,
            DataType::I16 | DataType::U16 => 2,
            DataType::I32 | DataType::U32 => 4,
            DataType::F32 => 4,
        }
    }
}

impl From<DataType> for GLenum {
    fn from(value: DataType) -> Self {
        match value {
            DataType::I8 => gl::BYTE,
            DataType::U8 => gl::UNSIGNED_BYTE,
            DataType::I16 => gl::SHORT,
            DataType::U16 => gl::UNSIGNED_SHORT,
            DataType::I32 => gl::INT,
            DataType::U32 => gl::UNSIGNED_INT,
            DataType::F32 => gl::FLOAT,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DrawMode {
    Points,
    Triangles,
}

pub fn draw_arrays(mode: DrawMode, starting_index: u64, count: u64) -> Result<(), Error> {
    unsafe { gl::DrawArrays(mode.into(), starting_index as _, count as _) };
    assert_no_error()
}

pub fn draw_arrays_instanced(
    mode: DrawMode,
    starting_index: u64,
    count: u64,
    instances: u32,
) -> Result<(), Error> {
    unsafe {
        gl::DrawArraysInstanced(mode.into(), starting_index as _, count as _, instances as _)
    };
    assert_no_error()
}

pub fn draw_elements(mode: DrawMode, num_indices: u64, index_type: DataType) -> Result<(), Error> {
    unsafe {
        gl::DrawElements(
            mode.into(),
            num_indices as _,
            index_type.into(),
            std::ptr::null(),
        )
    };
    assert_no_error()
}

pub fn draw_elements_instanced(
    mode: DrawMode,
    num_indices: u64,
    index_type: DataType,
    instances: u32,
) -> Result<(), Error> {
    unsafe {
        gl::DrawElementsInstanced(
            mode.into(),
            num_indices as _,
            index_type.into(),
            std::ptr::null(),
            instances as _,
        )
    };
    assert_no_error()
}

impl From<DrawMode> for GLenum {
    fn from(value: DrawMode) -> Self {
        match value {
            DrawMode::Points => gl::POINTS,
            DrawMode::Triangles => gl::TRIANGLES,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    Vertex,
    Geometry,
    Fragment,
    Compute,
}

pub type ShaderID = GLuint;

pub fn create_shader(shader_type: ShaderType) -> ShaderID {
    unsafe { gl::CreateShader(shader_type.into()) }
}

pub fn delete_shader(shader_id: ShaderID) -> Result<(), Error> {
    unsafe { gl::DeleteShader(shader_id) };
    assert_no_error()
}

pub fn set_shader_source(id: ShaderID, source: &str) -> Result<(), Error> {
    let source_str = CString::new(source).map_err(|_| Error::InvalidValue)?;
    unsafe {
        gl::ShaderSource(
            id,
            1,
            &source_str.as_c_str().as_ptr() as _,
            std::ptr::null(),
        )
    };
    Ok(assert_no_error()?)
}

pub fn compile_shader(id: ShaderID) -> Result<(), Error> {
    unsafe { gl::CompileShader(id) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum ShaderParameter {
    ShaderType,
    DeleteStatus,
    CompileStatus,
    InfoLogLength,
    SourceLength,
}

pub fn get_shader_parameter(id: ShaderID, parameter: ShaderParameter) -> Result<u32, Error> {
    let mut res = 0;
    unsafe { gl::GetShaderiv(id, parameter.into(), &mut res) };
    with_check(res as u32)
}

pub fn get_shader_info_log(id: ShaderID, buffer_size: u32) -> Result<String, Error> {
    let buffer = " ".repeat(buffer_size as usize);
    let error = unsafe { CString::new(buffer).unwrap_unchecked() };
    unsafe {
        gl::GetShaderInfoLog(
            id,
            buffer_size as GLsizei,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        )
    };
    with_check_fn(|| error.to_string_lossy().into_owned())
}

pub fn get_shader_source(id: ShaderID, buffer_size: u32) -> Result<String, Error> {
    let buffer = " ".repeat(buffer_size as usize);
    let error = unsafe { CString::new(buffer).unwrap_unchecked() };
    unsafe {
        gl::GetShaderSource(
            id,
            buffer_size as GLsizei,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        )
    };
    with_check_fn(|| error.to_string_lossy().into_owned())
}

impl From<ShaderType> for GLenum {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        }
    }
}

impl TryFrom<u32> for ShaderType {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as GLenum {
            gl::VERTEX_SHADER => Ok(ShaderType::Vertex),
            gl::GEOMETRY_SHADER => Ok(ShaderType::Geometry),
            gl::FRAGMENT_SHADER => Ok(ShaderType::Fragment),
            gl::COMPUTE_SHADER => Ok(ShaderType::Compute),
            _ => Err(format!("Unknown Shader Type with value {value}")),
        }
    }
}

impl From<ShaderParameter> for GLenum {
    fn from(value: ShaderParameter) -> Self {
        match value {
            ShaderParameter::ShaderType => gl::SHADER_TYPE,
            ShaderParameter::DeleteStatus => gl::DELETE_STATUS,
            ShaderParameter::CompileStatus => gl::COMPILE_STATUS,
            ShaderParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ShaderParameter::SourceLength => gl::SHADER_SOURCE_LENGTH,
        }
    }
}

pub type ProgramID = GLuint;

pub fn create_program() -> ProgramID {
    unsafe { gl::CreateProgram() }
}

pub fn delete_program(program_id: ProgramID) -> Result<(), Error> {
    unsafe { gl::DeleteProgram(program_id) };
    assert_no_error()
}

pub fn attach_shader(program_id: ProgramID, shader_id: ShaderID) -> Result<(), Error> {
    unsafe { gl::AttachShader(program_id, shader_id) };
    assert_no_error()
}

pub fn detach_shader(program_id: ProgramID, shader_id: ShaderID) -> Result<(), Error> {
    unsafe { gl::DetachShader(program_id, shader_id) };
    assert_no_error()
}

pub fn link_program(program_id: ProgramID) -> Result<(), Error> {
    unsafe { gl::LinkProgram(program_id) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum ProgramParameter {
    LinkStatus,
    InfoLogLength,
    // TODO: many more
}

pub fn get_program_paramter(
    program_id: ProgramID,
    parameter: ProgramParameter,
) -> Result<u32, Error> {
    let mut res = 0;
    unsafe { gl::GetProgramiv(program_id, parameter.into(), &mut res) };
    with_check(res as u32)
}

pub fn get_program_info_log(id: ProgramID, buffer_size: u32) -> Result<String, Error> {
    let buffer = " ".repeat(buffer_size as usize);
    let error = unsafe { CString::new(buffer).unwrap_unchecked() };
    unsafe {
        gl::GetProgramInfoLog(
            id,
            buffer_size as GLsizei,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        )
    };
    with_check_fn(|| error.to_string_lossy().into_owned())
}

pub fn use_program(program_id: ProgramID) -> Result<(), Error> {
    unsafe { gl::UseProgram(program_id) }
    assert_no_error()
}

impl From<ProgramParameter> for GLenum {
    fn from(value: ProgramParameter) -> Self {
        match value {
            ProgramParameter::LinkStatus => gl::LINK_STATUS,
            ProgramParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
        }
    }
}

pub type UniformLocation = GLuint;

pub fn get_uniform_location(
    program_id: ProgramID,
    name: &str,
) -> Result<Option<UniformLocation>, Error> {
    let name_cstr = unsafe { CString::new(name).unwrap_unchecked() };

    let location =
        unsafe { gl::GetUniformLocation(program_id, name_cstr.as_c_str().as_ptr() as _) };

    with_check_fn(|| {
        if location < 0 {
            None
        } else {
            Some(location as u32)
        }
    })
}

pub fn set_uniform_i32(location: UniformLocation, data: i32) -> Result<(), Error> {
    unsafe { gl::Uniform1i(location as _, data) };
    assert_no_error()
}

pub fn set_uniform_f32(location: UniformLocation, data: f32) -> Result<(), Error> {
    unsafe { gl::Uniform1f(location as _, data) };
    assert_no_error()
}

pub fn set_uniform_vec2(location: UniformLocation, data: &[f32; 2]) -> Result<(), Error> {
    unsafe { gl::Uniform2fv(location as _, 1, data.as_ptr()) };
    assert_no_error()
}

pub fn set_uniform_vec3(location: UniformLocation, data: &[f32; 3]) -> Result<(), Error> {
    unsafe { gl::Uniform3fv(location as _, 1, data.as_ptr()) };
    assert_no_error()
}

pub fn set_uniform_mat4(
    location: UniformLocation,
    transpose: bool,
    data: &[f32],
) -> Result<(), Error> {
    unsafe { gl::UniformMatrix4fv(location as _, 1, bool_to_enum(transpose), data.as_ptr()) };
    assert_no_error()
}

pub type BufferID = GLuint;

pub fn create_buffer() -> BufferID {
    let mut id = 0;
    unsafe { gl::GenBuffers(1, &mut id) };
    id
}

pub fn delete_buffer(buffer_id: BufferID) -> Result<(), Error> {
    unsafe { gl::DeleteBuffers(1, &buffer_id) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum BufferTarget {
    Array,
    ElementArray,
}

pub fn bind_buffer(buffer_id: BufferID, buffer_target: BufferTarget) -> Result<(), Error> {
    unsafe { gl::BindBuffer(buffer_target.into(), buffer_id) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
    StaticDraw,
}

pub fn set_buffer_data<Data>(
    buffer_target: BufferTarget,
    usage: BufferUsage,
    data: &[Data],
) -> Result<(), Error> {
    unsafe {
        gl::BufferData(
            buffer_target.into(),
            (data.len() * std::mem::size_of::<Data>()) as _,
            data.as_ptr() as _,
            usage.into(),
        )
    };
    assert_no_error()
}

impl From<BufferTarget> for GLenum {
    fn from(value: BufferTarget) -> Self {
        match value {
            BufferTarget::Array => gl::ARRAY_BUFFER,
            BufferTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl From<BufferUsage> for GLenum {
    fn from(value: BufferUsage) -> Self {
        match value {
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
        }
    }
}

pub type VertexArrayID = GLuint;

pub fn create_vertex_array() -> VertexArrayID {
    let mut id = 0;
    unsafe { gl::GenVertexArrays(1, &mut id) };
    id
}

pub fn delete_vertex_array(vertex_array_id: VertexArrayID) -> Result<(), Error> {
    unsafe { gl::DeleteVertexArrays(1, &vertex_array_id) };
    assert_no_error()
}

pub fn bind_vertex_array(vertex_array_id: VertexArrayID) -> Result<(), Error> {
    unsafe { gl::BindVertexArray(vertex_array_id) };
    assert_no_error()
}

pub fn enable_vertex_attribute_array(index: u32) -> Result<(), Error> {
    unsafe { gl::EnableVertexAttribArray(index) };
    assert_no_error()
}

pub fn set_vertex_attribute_divisor(attribute_index: u32, divisor: u32) -> Result<(), Error> {
    unsafe { gl::VertexAttribDivisor(attribute_index, divisor) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum VertexAttributeSize {
    Single,
    Double,
    Triple,
    Quadruple,
    BGRA,
}

pub fn vertex_attribute_pointer(
    index: u32,
    component_size: VertexAttributeSize,
    data_type: DataType,
    normalised: bool,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    unsafe {
        gl::VertexAttribPointer(
            index as _,
            component_size.into(),
            data_type.into(),
            bool_to_enum(normalised),
            stride as _,
            offset as _,
        )
    };
    assert_no_error()
}

impl VertexAttributeSize {
    pub fn as_value(&self) -> u32 {
        match self {
            VertexAttributeSize::Single => 1,
            VertexAttributeSize::Double => 2,
            VertexAttributeSize::Triple => 3,
            VertexAttributeSize::Quadruple => 4,
            VertexAttributeSize::BGRA => gl::BGRA as _,
        }
    }
}

impl From<VertexAttributeSize> for GLint {
    fn from(value: VertexAttributeSize) -> Self {
        value.as_value() as _
    }
}

pub type TextureID = GLuint;

pub fn create_texture() -> TextureID {
    let mut id = 0;
    unsafe { gl::GenTextures(1, &mut id) };
    id
}

pub fn delete_texture(texture_id: TextureID) -> Result<(), Error> {
    unsafe { gl::DeleteTextures(1, &texture_id) }
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum TextureCubeMapFaceTarget {
    Right,
    Left,
    Top,
    Bottom,
    Back,
    Front,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureTarget {
    Image2D,
    CubeMap,
    CubeMapFace(TextureCubeMapFaceTarget),
}

pub fn bind_texture(texture_id: TextureID, texture_target: TextureTarget) -> Result<(), Error> {
    unsafe { gl::BindTexture(texture_target.into(), texture_id) };
    assert_no_error()
}

pub fn active_texture(texture_index: u32) -> Result<(), Error> {
    unsafe { gl::ActiveTexture(gl::TEXTURE0 + texture_index) };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum DepthFunc {
    Less,
    LessEqual,
}

pub fn set_depth_func(func: DepthFunc) {
    unsafe { gl::DepthFunc(func.into()) };
}

impl From<DepthFunc> for GLenum {
    fn from(value: DepthFunc) -> Self {
        match value {
            DepthFunc::Less => gl::LESS,
            DepthFunc::LessEqual => gl::LEQUAL,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextureParameterName {
    WrapS,
    WrapT,
    WrapR,
    MinFilter,
    MagFilter,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureParameterValue {
    Linear,
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

pub fn set_texture_parameter_value(
    texture_target: TextureTarget,
    texture_parameter_name: TextureParameterName,
    texture_parameter_value: TextureParameterValue,
) -> Result<(), Error> {
    unsafe {
        gl::TexParameteri(
            texture_target.into(),
            texture_parameter_name.into(),
            texture_parameter_value.into(),
        )
    };
    assert_no_error()
}

#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
    RGB,
    RGBA,
}

#[allow(clippy::too_many_arguments)]
pub fn load_texture_image2d<Data>(
    texture_target: TextureTarget,
    mipmap_level: u32,
    internal_format: TextureFormat,
    width: u64,
    height: u64,
    data_format: TextureFormat,
    data_type: DataType,
    data: Option<&[Data]>,
) -> Result<(), Error> {
    let internal_format: GLenum = internal_format.into();
    unsafe {
        gl::TexImage2D(
            texture_target.into(),
            mipmap_level as _,
            internal_format as _,
            width as _,
            height as _,
            0,
            data_format.into(),
            data_type.into(),
            data.map_or(std::ptr::null(), |bytes| bytes.as_ptr()) as _,
        )
    };
    assert_no_error()
}

pub fn generate_mipmaps(texture_target: TextureTarget) -> Result<(), Error> {
    unsafe { gl::GenerateMipmap(texture_target.into()) };
    assert_no_error()
}

impl From<TextureTarget> for GLuint {
    fn from(value: TextureTarget) -> Self {
        match value {
            TextureTarget::Image2D => gl::TEXTURE_2D,
            TextureTarget::CubeMap => gl::TEXTURE_CUBE_MAP,
            TextureTarget::CubeMapFace(face_target) => face_target.into(),
        }
    }
}

impl From<TextureCubeMapFaceTarget> for GLenum {
    fn from(value: TextureCubeMapFaceTarget) -> Self {
        match value {
            TextureCubeMapFaceTarget::Right => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            TextureCubeMapFaceTarget::Left => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            TextureCubeMapFaceTarget::Top => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            TextureCubeMapFaceTarget::Bottom => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            TextureCubeMapFaceTarget::Front => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            TextureCubeMapFaceTarget::Back => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        }
    }
}

impl From<TextureParameterName> for GLenum {
    fn from(value: TextureParameterName) -> Self {
        match value {
            TextureParameterName::WrapS => gl::TEXTURE_WRAP_S,
            TextureParameterName::WrapT => gl::TEXTURE_WRAP_T,
            TextureParameterName::WrapR => gl::TEXTURE_WRAP_R,
            TextureParameterName::MinFilter => gl::TEXTURE_MIN_FILTER,
            TextureParameterName::MagFilter => gl::TEXTURE_MAG_FILTER,
        }
    }
}

impl From<TextureParameterValue> for GLint {
    fn from(value: TextureParameterValue) -> Self {
        (match value {
            TextureParameterValue::Linear => gl::LINEAR,
            TextureParameterValue::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureParameterValue::Repeat => gl::REPEAT,
            TextureParameterValue::MirroredRepeat => gl::MIRRORED_REPEAT,
        }) as _
    }
}

impl From<TextureFormat> for GLenum {
    fn from(value: TextureFormat) -> Self {
        match value {
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA => gl::RGBA,
        }
    }
}

pub type FrameBufferID = GLuint;

pub fn create_frame_buffer() -> FrameBufferID {
    let mut id = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut id);
    }
    id
}

pub enum FrameBufferTarget {
    All,
    Read,
    Draw,
}

pub fn bind_frame_buffer(id: FrameBufferID, target: FrameBufferTarget) -> Result<(), Error> {
    unsafe { gl::BindFramebuffer(target.into(), id) };
    assert_no_error()
}

pub enum FrameBufferAttachment {
    Colour(u32),
    DepthStencil,
}

pub fn frame_buffer_texture_2d(
    framebuffer_target: FrameBufferTarget,
    attachment: FrameBufferAttachment,
    texture_target: TextureTarget,
    texture_id: TextureID,
    mipmap_level: u32,
) -> Result<(), Error> {
    unsafe {
        gl::FramebufferTexture2D(
            framebuffer_target.into(),
            attachment.into(),
            texture_target.into(),
            texture_id,
            mipmap_level as _,
        )
    };
    assert_no_error()
}

impl From<FrameBufferTarget> for GLenum {
    fn from(value: FrameBufferTarget) -> Self {
        match value {
            FrameBufferTarget::All => gl::FRAMEBUFFER,
            FrameBufferTarget::Read => gl::READ_FRAMEBUFFER,
            FrameBufferTarget::Draw => gl::DRAW_FRAMEBUFFER,
        }
    }
}

impl From<FrameBufferAttachment> for GLenum {
    fn from(value: FrameBufferAttachment) -> Self {
        match value {
            FrameBufferAttachment::Colour(index) => gl::COLOR_ATTACHMENT0 + index,
            FrameBufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

pub type RenderBufferID = GLuint;

pub fn create_render_buffer() -> RenderBufferID {
    let mut id = 0;
    unsafe { gl::GenRenderbuffers(1, &mut id) };
    id
}

pub fn bind_render_buffer(id: RenderBufferID) -> Result<(), Error> {
    unsafe { gl::BindRenderbuffer(gl::RENDERBUFFER, id) };
    assert_no_error()
}

pub enum RenderBufferStorageFormat {
    Depth24Stencil8,
}

pub fn render_buffer_storage(
    internal_format: RenderBufferStorageFormat,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    unsafe {
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            internal_format.into(),
            width as _,
            height as _,
        )
    };
    assert_no_error()
}

impl From<RenderBufferStorageFormat> for GLenum {
    fn from(value: RenderBufferStorageFormat) -> Self {
        match value {
            RenderBufferStorageFormat::Depth24Stencil8 => gl::DEPTH24_STENCIL8,
        }
    }
}

pub fn frame_buffer_render_buffer(
    target: FrameBufferTarget,
    attachment: FrameBufferAttachment,
    render_buffer_id: RenderBufferID,
) -> Result<(), Error> {
    unsafe {
        gl::FramebufferRenderbuffer(
            target.into(),
            attachment.into(),
            gl::RENDERBUFFER,
            render_buffer_id,
        )
    };
    assert_no_error()
}

pub enum Feature {
    DepthTest,
}

pub fn enable(feature: Feature) -> Result<(), Error> {
    unsafe { gl::Enable(feature.into()) };
    assert_no_error()
}

pub fn disable(feature: Feature) -> Result<(), Error> {
    unsafe { gl::Disable(feature.into()) };
    assert_no_error()
}

impl From<Feature> for GLenum {
    fn from(value: Feature) -> Self {
        match value {
            Feature::DepthTest => gl::DEPTH_TEST,
        }
    }
}

pub enum BufferBit {
    Colour,
    Depth,
}

pub fn clear(buffer: BufferBit) -> Result<(), Error> {
    unsafe { gl::Clear(buffer.into()) };
    assert_no_error()
}

pub fn clear_colour(r: f32, g: f32, b: f32, a: f32) -> Result<(), Error> {
    unsafe { gl::ClearColor(r, g, b, a) };
    assert_no_error()
}

impl From<BufferBit> for GLenum {
    fn from(value: BufferBit) -> Self {
        match value {
            BufferBit::Colour => gl::COLOR_BUFFER_BIT,
            BufferBit::Depth => gl::DEPTH_BUFFER_BIT,
        }
    }
}

fn bool_to_enum(value: bool) -> GLboolean {
    match value {
        true => gl::TRUE,
        false => gl::FALSE,
    }
}

fn with_check<Data>(data: Data) -> Result<Data, Error> {
    match get_error() {
        Some(error) => Err(error),
        None => Ok(data),
    }
}

fn with_check_fn<F, R>(f: F) -> Result<R, Error>
where
    F: FnOnce() -> R,
{
    match get_error() {
        Some(error) => Err(error),
        None => Ok(f()),
    }
}
