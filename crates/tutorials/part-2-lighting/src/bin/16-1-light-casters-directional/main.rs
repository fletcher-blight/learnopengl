use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("1.10-camera", 1920, 1080)?;

    let object_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("object.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let image_dir = asset_dir.join("images");
    let container_texture =
        opengl::TextureImage2D::load_from_file(&image_dir.join("container2.png"))?;
    let container_specular_texture =
        opengl::TextureImage2D::load_from_file(&image_dir.join("container2_specular.png"))?;

    let container_shader_texture = opengl::ShaderProgramTexture::new(
        &container_texture,
        &object_shader_program,
        "material.diffuse",
        0,
    )?;
    let container_specular_shader_texture = opengl::ShaderProgramTexture::new(
        &container_specular_texture,
        &object_shader_program,
        "material.specular",
        1,
    )?;

    #[rustfmt::skip]
        let vertices = [
        // front
        [-0.5f32, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],

        // left
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],

        // top
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],

        // right
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],

        // bottom
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],

        // back
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
    ];

    let object_positions = [
        [0.0, 0.0, 0.0],
        [2.0, 5.0, -15.0],
        [-1.5, -2.2, -2.5],
        [-3.8, -2.0, -12.3],
        [2.4, -0.4, -3.5],
        [-1.7, 3.0, -7.5],
        [1.3, -2.0, -2.5],
        [1.5, 2.0, -2.5],
        [1.5, 0.2, -1.5],
        [-1.3, 1.0, -1.5f32],
    ];

    let mesh: opengl::Mesh = vertices.as_slice().try_into()?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    object_shader_program.enable()?;
    let object_model_location = object_shader_program.locate_uniform("model")?;
    let object_view_location = object_shader_program.locate_uniform("view")?;
    let object_projection_location = object_shader_program.locate_uniform("projection")?;
    let object_view_pos_location = object_shader_program.locate_uniform("view_pos")?;
    let object_material_shininess_location =
        object_shader_program.locate_uniform("material.shininess")?;
    let object_light_direction_location =
        object_shader_program.locate_uniform("light.direction")?;
    let object_light_ambient_location = object_shader_program.locate_uniform("light.ambient")?;
    let object_light_diffuse_location = object_shader_program.locate_uniform("light.diffuse")?;
    let object_light_specular_location = object_shader_program.locate_uniform("light.specular")?;

    opengl_sys::set_uniform_vec3(object_light_ambient_location, &[0.2, 0.2, 0.2])?;
    opengl_sys::set_uniform_vec3(object_light_diffuse_location, &[0.5, 0.5, 0.5])?;
    opengl_sys::set_uniform_vec3(object_light_specular_location, &[1.0, 1.0, 1.0])?;
    opengl_sys::set_uniform_f32(object_material_shininess_location, 32.0)?;
    opengl_sys::set_uniform_vec3(object_light_direction_location, &[-0.2, -1.0, -0.3]).unwrap();

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
        set_mat4(object_view_location, &camera.calculate_view());
        opengl_sys::set_uniform_vec3(object_view_pos_location, &camera.get_position()).unwrap();
        set_mat4(
            object_projection_location,
            &camera.calculate_projection(window_size),
        );

        container_shader_texture.draw().unwrap();
        container_specular_shader_texture.draw().unwrap();
        for (index, [x, y, z]) in object_positions.iter().enumerate() {
            set_mat4(
                object_model_location,
                &glm::rotate(
                    &glm::translate(&glm::one(), &glm::vec3(*x, *y, *z)),
                    (20.0 * index as f32).to_radians(),
                    &glm::vec3(1.0, 0.3, 0.5),
                ),
            );
            mesh.draw(opengl::DrawMode::Triangles).unwrap();
        }
    })
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}
