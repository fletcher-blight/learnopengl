use opengl_sys::{Feature, RenderBufferID, TextureID};

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("4.26 Frame Buffers", 1920, 1080)?;

    let object_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("object.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("object.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let flavour_location = object_shader_program.locate_uniform("flavour")?;

    let frame_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("frame.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("frame.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let triangle_mesh: opengl::Mesh =
        ([[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]].as_slice()).try_into()?;
    let quad_mesh: opengl::Mesh = ([
        [-1.0, -1.0],
        [-1.0, 1.0],
        [1.0, 1.0],
        [-1.0, -1.0],
        [1.0, 1.0],
        [1.0, -1.0],
    ]
    .as_slice())
    .try_into()?;

    let frame_buffer1_id = opengl_sys::create_frame_buffer();
    opengl_sys::bind_frame_buffer(frame_buffer1_id, opengl_sys::FrameBufferTarget::All)?;
    let texture_image1_id = create_render_frame()?;
    let render_buffer1_id = create_render_buffer()?;
    {
        frame_shader_program.enable()?;
        let location = frame_shader_program.locate_uniform("tex1")?;
        opengl_sys::set_uniform_i32(location, 0)?;
    }

    let frame_buffer2_id = opengl_sys::create_frame_buffer();
    opengl_sys::bind_frame_buffer(frame_buffer2_id, opengl_sys::FrameBufferTarget::All)?;
    let texture_image2_id = create_render_frame()?;
    let render_buffer2_id = create_render_buffer()?;
    {
        frame_shader_program.enable()?;
        let location = frame_shader_program.locate_uniform("tex2")?;
        opengl_sys::set_uniform_i32(location, 1)?;
    }

    window.run(|_, _, _| {
        // ===================== Render Frame Buffers

        // ===================== Render First Frame Buffer
        opengl_sys::bind_frame_buffer(frame_buffer1_id, opengl_sys::FrameBufferTarget::All)
            .unwrap();
        opengl_sys::enable(Feature::DepthTest).unwrap();

        opengl_sys::clear_colour(0.0, 0.0, 0.0, 1.0).unwrap();
        opengl_sys::clear(opengl_sys::BufferBit::Colour).unwrap();
        opengl_sys::clear(opengl_sys::BufferBit::Depth).unwrap();

        object_shader_program.enable().unwrap();
        opengl_sys::set_uniform_vec3(flavour_location, &[1.0, 0.0, 0.0]).unwrap();
        opengl_sys::frame_buffer_texture_2d(
            opengl_sys::FrameBufferTarget::All,
            opengl_sys::FrameBufferAttachment::Colour(0),
            opengl_sys::TextureTarget::Image2D,
            texture_image1_id,
            0,
        )
        .unwrap();
        triangle_mesh.draw(opengl::DrawMode::Triangles).unwrap();

        // ===================== Render Second Frame Buffer

        opengl_sys::bind_frame_buffer(frame_buffer2_id, opengl_sys::FrameBufferTarget::All)
            .unwrap();
        opengl_sys::enable(Feature::DepthTest).unwrap();

        opengl_sys::clear_colour(0.0, 0.0, 0.0, 1.0).unwrap();
        opengl_sys::clear(opengl_sys::BufferBit::Colour).unwrap();
        opengl_sys::clear(opengl_sys::BufferBit::Depth).unwrap();

        object_shader_program.enable().unwrap();
        opengl_sys::set_uniform_vec3(flavour_location, &[0.0, 1.0, 0.0]).unwrap();
        opengl_sys::frame_buffer_texture_2d(
            opengl_sys::FrameBufferTarget::All,
            opengl_sys::FrameBufferAttachment::Colour(0),
            opengl_sys::TextureTarget::Image2D,
            texture_image2_id,
            0,
        )
        .unwrap();
        triangle_mesh.draw(opengl::DrawMode::Triangles).unwrap();

        // ===================== Render to Display

        opengl_sys::bind_frame_buffer(0, opengl_sys::FrameBufferTarget::All).unwrap();
        opengl_sys::disable(Feature::DepthTest).unwrap();

        opengl_sys::clear_colour(0.0, 0.0, 0.0, 1.0).unwrap();
        opengl_sys::clear(opengl_sys::BufferBit::Colour).unwrap();

        frame_shader_program.enable().unwrap();
        opengl_sys::active_texture(0).unwrap();
        opengl_sys::bind_texture(texture_image1_id, opengl_sys::TextureTarget::Image2D).unwrap();
        opengl_sys::active_texture(1).unwrap();
        opengl_sys::bind_texture(texture_image2_id, opengl_sys::TextureTarget::Image2D).unwrap();
        quad_mesh.draw(opengl::DrawMode::Triangles).unwrap();
    })
}

fn create_render_frame() -> anyhow::Result<TextureID> {
    let texture_image_id = opengl_sys::create_texture();
    opengl_sys::bind_texture(texture_image_id, opengl_sys::TextureTarget::Image2D)?;
    opengl_sys::load_texture_image2d(
        opengl_sys::TextureTarget::Image2D,
        0,
        opengl_sys::TextureFormat::RGB,
        1920,
        1080,
        opengl_sys::TextureFormat::RGB,
        opengl_sys::DataType::U32,
        None as Option<&[()]>,
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
    Ok(texture_image_id)
}

fn create_render_buffer() -> anyhow::Result<RenderBufferID> {
    let render_buffer_id = opengl_sys::create_render_buffer();
    opengl_sys::bind_render_buffer(render_buffer_id)?;
    opengl_sys::render_buffer_storage(
        opengl_sys::RenderBufferStorageFormat::Depth24Stencil8,
        1920,
        1080,
    )?;
    opengl_sys::frame_buffer_render_buffer(
        opengl_sys::FrameBufferTarget::All,
        opengl_sys::FrameBufferAttachment::DepthStencil,
        render_buffer_id,
    )?;
    Ok(render_buffer_id)
}
