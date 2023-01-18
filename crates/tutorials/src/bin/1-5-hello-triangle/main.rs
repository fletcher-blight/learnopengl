fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.5 Hello, Triangle", 1920, 1080)?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let mesh = opengl::Mesh::create_and_bind(
        &[[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]],
        &[opengl::BufferAttributeSize::Double.into()],
        None,
        opengl_sys::DrawMode::Triangles,
    )?;

    window.run(|_, _, _| {
        shader_program.enable().unwrap();
        mesh.draw().unwrap();
    })
}
