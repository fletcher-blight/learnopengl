fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.5 Hello, Triangle", 1920, 1080)?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let mesh: opengl::Mesh = ([[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]].as_slice()).try_into()?;

    window.run(|_, _, _| {
        shader_program.enable().unwrap();
        mesh.draw(opengl::DrawMode::Triangles).unwrap();
    })
}
