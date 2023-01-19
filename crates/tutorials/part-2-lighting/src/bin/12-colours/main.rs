use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.10-camera", 1920, 1080)?;

    let object_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("object.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let light_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("light.frag"), opengl::ShaderType::Fragment)?,
    ])?;

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

    let mesh: opengl::Mesh = vertices.as_slice().try_into()?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    object_shader_program.enable()?;
    let object_model_location = object_shader_program.locate_uniform("model")?;
    let object_view_location = object_shader_program.locate_uniform("view")?;
    let object_projection_location = object_shader_program.locate_uniform("projection")?;
    let object_colour_location = object_shader_program.locate_uniform("object_colour")?;
    let light_colour_location = object_shader_program.locate_uniform("light_colour")?;
    opengl_sys::set_uniform_vec3(object_colour_location, &[1.0, 0.5, 0.31])?;
    opengl_sys::set_uniform_vec3(light_colour_location, &[1.0, 1.0, 1.0])?;

    let light_pos = glm::vec3(1.2, 1.0, 2.0);
    light_shader_program.enable()?;
    let light_model_location = light_shader_program.locate_uniform("model")?;
    let light_view_location = light_shader_program.locate_uniform("view")?;
    let light_projection_location = light_shader_program.locate_uniform("projection")?;

    window.run(|window_size, (_, seconds_since_last_frame), events| {
        camera::process_events(
            &mut camera,
            &mut camera_controls,
            70.0,
            0.97,
            seconds_since_last_frame,
            events,
        );

        object_shader_program.enable().unwrap();
        set_mat4(object_model_location, &glm::one());
        set_mat4(object_view_location, &camera.calculate_view());
        set_mat4(
            object_projection_location,
            &camera.calculate_projection(window_size),
        );
        mesh.draw(opengl::DrawMode::Triangles).unwrap();

        light_shader_program.enable().unwrap();
        set_mat4(
            light_model_location,
            &glm::scale(
                &glm::translate(&glm::one(), &light_pos),
                &glm::vec3(0.2, 0.2, 0.2),
            ),
        );
        set_mat4(light_view_location, &camera.calculate_view());
        set_mat4(
            light_projection_location,
            &camera.calculate_projection(window_size),
        );
        mesh.draw(opengl::DrawMode::Triangles).unwrap();
    })
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}
