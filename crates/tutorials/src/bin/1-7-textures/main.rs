fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.5 Hello, Triangle", 1920, 1080)?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let image_dir = asset_dir.join("images");
    let texture = opengl::TextureImage2D::load_from_file(&image_dir.join("splatoon-face.jpeg"))?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let shader_texture = opengl::ShaderProgramTexture::new(&texture, &shader_program, "tex", 0)?;

    let mesh = opengl::Mesh::create_and_bind(
        &[[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]],
        &[opengl::BufferAttribute {
            size: opengl_sys::VertexAttributeSize::Double,
            data_type: opengl_sys::DataType::F32,
        }],
        None,
        opengl_sys::DrawMode::Triangles,
    )?;

    window.run(|_, _, _| {
        shader_program.enable().unwrap();
        shader_texture.draw().unwrap();
        mesh.draw().unwrap();
    })
}
