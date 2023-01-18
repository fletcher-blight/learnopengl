use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.10 Camera", 1920, 1080)?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let image_dir = asset_dir.join("images");
    let texture = opengl::TextureImage2D::load_from_file(&image_dir.join("splatoon-face.jpeg"))?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let shader_texture = opengl::ShaderProgramTexture::new(&texture, &shader_program, "tex", 0)?;

    #[rustfmt::skip]
    let vertices = [
        [-0.5f32, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],

        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],

        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],

        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],

        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
    ];

    let mesh = opengl::Mesh::create_and_bind(
        &vertices,
        &[opengl::BufferAttributeSize::Triple.into()],
        None,
        opengl_sys::DrawMode::Triangles,
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

        opengl_sys::set_uniform_mat4(
            model_location,
            false,
            glm::value_ptr::<f32, 4, 4>(&glm::one()),
        )
        .unwrap();
        opengl_sys::set_uniform_mat4(
            view_location,
            false,
            glm::value_ptr(&camera.calculate_view()),
        )
        .unwrap();
        opengl_sys::set_uniform_mat4(
            projection_location,
            false,
            glm::value_ptr(&camera.calculate_projection(window_size)),
        )
        .unwrap();

        mesh.draw().unwrap();
    })
}
