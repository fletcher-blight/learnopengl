use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("17 - Multiple Lights", 1920, 1080)?;

    let object_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("object.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let light_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("light.frag"), opengl::ShaderType::Fragment)?,
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
    opengl_sys::set_uniform_f32(object_material_shininess_location, 32.0)?;

    // ======================== Directional Light ========================

    {
        let object_dir_light_direction_location =
            object_shader_program.locate_uniform("dir_light.direction")?;
        let object_dir_light_ambient_location =
            object_shader_program.locate_uniform("dir_light.ambient")?;
        let object_dir_light_diffuse_location =
            object_shader_program.locate_uniform("dir_light.diffuse")?;
        let object_dir_light_specular_location =
            object_shader_program.locate_uniform("dir_light.specular")?;

        opengl_sys::set_uniform_vec3(object_dir_light_ambient_location, &[0.05, 0.05, 0.05])?;
        opengl_sys::set_uniform_vec3(object_dir_light_diffuse_location, &[0.4, 0.4, 0.4])?;
        opengl_sys::set_uniform_vec3(object_dir_light_specular_location, &[0.5, 0.5, 0.5])?;
        opengl_sys::set_uniform_vec3(object_dir_light_direction_location, &[-0.2, -1.0, -0.3])
            .unwrap();
    }

    // ======================== Point Lights ========================

    #[rustfmt::skip]
    let point_light_positions = [
        [0.2, 0.7, 2.0],
        [2.3, -3.3, -4.0],
        [-4.0, 2.0, -12.0],
        [0.0, 0.0, -3.0f32],
    ];

    for (index, position) in point_light_positions.iter().enumerate() {
        let position_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].position").as_str())?;
        let ambient_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].ambient").as_str())?;
        let diffuse_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].diffuse").as_str())?;
        let specular_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].specular").as_str())?;
        let attenuation_linear_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].attenuation_linear").as_str())?;
        let attenuation_quadratic_location = object_shader_program
            .locate_uniform(format!("point_lights[{index}].attenuation_quadratic").as_str())?;

        opengl_sys::set_uniform_vec3(ambient_location, &[0.05, 0.05, 0.05])?;
        opengl_sys::set_uniform_vec3(diffuse_location, &[0.8, 0.8, 0.8])?;
        opengl_sys::set_uniform_vec3(specular_location, &[1.0, 1.0, 1.0])?;
        opengl_sys::set_uniform_vec3(position_location, &position)?;
        opengl_sys::set_uniform_f32(attenuation_linear_location, 0.09)?;
        opengl_sys::set_uniform_f32(attenuation_quadratic_location, 0.032)?;
    }

    // ======================== Spot Light ========================

    let object_spot_light_position_location =
        object_shader_program.locate_uniform("spot_light.position")?;
    let object_spot_light_direction_location =
        object_shader_program.locate_uniform("spot_light.direction")?;

    {
        let object_spot_light_cutoff_location =
            object_shader_program.locate_uniform("spot_light.cutoff")?;
        let object_spot_light_outer_cutoff_location =
            object_shader_program.locate_uniform("spot_light.outer_cutoff")?;
        let object_spot_light_ambient_location =
            object_shader_program.locate_uniform("spot_light.ambient")?;
        let object_spot_light_diffuse_location =
            object_shader_program.locate_uniform("spot_light.diffuse")?;
        let object_spot_light_specular_location =
            object_shader_program.locate_uniform("spot_light.specular")?;
        let object_spot_light_attenuation_linear_location =
            object_shader_program.locate_uniform("spot_light.attenuation_linear")?;
        let object_spot_light_attenuation_quadratic_location =
            object_shader_program.locate_uniform("spot_light.attenuation_quadratic")?;

        opengl_sys::set_uniform_vec3(object_spot_light_ambient_location, &[0.0, 0.0, 0.0])?;
        opengl_sys::set_uniform_vec3(object_spot_light_diffuse_location, &[1.0, 1.0, 1.0])?;
        opengl_sys::set_uniform_vec3(object_spot_light_specular_location, &[1.0, 1.0, 1.0])?;
        opengl_sys::set_uniform_f32(
            object_spot_light_cutoff_location,
            12.5f32.to_radians().cos(),
        )?;
        opengl_sys::set_uniform_f32(
            object_spot_light_outer_cutoff_location,
            15.0f32.to_radians().cos(),
        )?;
        opengl_sys::set_uniform_f32(object_spot_light_attenuation_linear_location, 0.09)?;
        opengl_sys::set_uniform_f32(object_spot_light_attenuation_quadratic_location, 0.032)?;
    }

    // ======================== Light Shader ========================

    light_shader_program.enable()?;
    let light_model_location = light_shader_program.locate_uniform("model")?;
    let light_view_location = light_shader_program.locate_uniform("view")?;
    let light_projection_location = light_shader_program.locate_uniform("projection")?;

    // ======================== Loop ========================

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
        opengl_sys::set_uniform_vec3(object_spot_light_position_location, &camera.get_position())
            .unwrap();
        opengl_sys::set_uniform_vec3(
            object_spot_light_direction_location,
            &camera.get_direction(),
        )
        .unwrap();

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

        light_shader_program.enable().unwrap();
        for [x, y, z] in point_light_positions {
            set_mat4(
                light_model_location,
                &glm::scale(
                    &glm::translate(&glm::one(), &glm::vec3(x, y, z)),
                    &glm::vec3(0.2, 0.2, 0.2),
                ),
            );
            set_mat4(light_view_location, &camera.calculate_view());
            set_mat4(
                light_projection_location,
                &camera.calculate_projection(window_size),
            );
            mesh.draw(opengl::DrawMode::Triangles).unwrap();
        }
    })
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}
