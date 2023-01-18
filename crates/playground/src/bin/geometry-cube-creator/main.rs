use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.5 Hello, Triangle", 1920, 1080)?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let image_dir = asset_dir.join("images");
    let texture = opengl::TextureImage2D::load_from_file(&image_dir.join("splatoon-face.jpeg"))?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.geom"), opengl::ShaderType::Geometry)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let shader_texture = opengl::ShaderProgramTexture::new(&texture, &shader_program, "tex", 0)?;

    let mesh = opengl::Mesh::create_and_bind(
        &[
            [0.0f32, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [-10.0, 0.0, 0.0],
            [0.0, 10.0, 0.0],
            [0.0, -10.0, 0.0],
            [0.0, 0.0, 10.0],
            [0.0, 0.0, -10.0],
        ],
        &[opengl::BufferAttributeSize::Triple.into()],
        None,
        opengl_sys::DrawMode::Points,
    )?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    let model_location = shader_program.locate_uniform("model")?;
    let view_location = shader_program.locate_uniform("view")?;
    let projection_location = shader_program.locate_uniform("projection")?;

    window.run(|window_size, (_, seconds_since_last_frame), events| {
        camera::process_events(
            &mut camera,
            &mut camera_controls,
            70.0,
            0.97,
            seconds_since_last_frame,
            events,
        );

        shader_program.enable().unwrap();
        shader_texture.draw().unwrap();

        set_mat4(model_location, &glm::one());
        set_mat4(view_location, &camera.calculate_view());
        set_mat4(
            projection_location,
            &camera.calculate_projection(window_size),
        );

        mesh.draw().unwrap();
    })
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}
