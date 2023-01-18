use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("Playground: Skybox Spinning Cube Land", 1920, 1080)?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("skybox.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("skybox.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let asset_dir = std::env::current_dir()?.join("assets");
    // let image_dir = asset_dir.join("images");
    let skybox_dir = asset_dir.join("skyboxes");

    // let cube_texture =
    //     opengl::TextureImage2D::load_from_file(&image_dir.join("splatoon-face.jpeg"))?;
    let skybox_texture = opengl::TextureCubeMap::load_from_file(&opengl::CubeMap {
        right: skybox_dir.join("right.jpg"),
        left: skybox_dir.join("left.jpg"),
        top: skybox_dir.join("top.jpg"),
        bottom: skybox_dir.join("bottom.jpg"),
        back: skybox_dir.join("back.jpg"),
        front: skybox_dir.join("front.jpg"),
    })?;

    // let cube_shader_texture =
    //     opengl::ShaderProgramTexture::new(&cube_texture, &shader_program, "tex", 0)?;
    let skybox_shader_texture =
        opengl::ShaderProgramTexture::new(&skybox_texture, &shader_program, "skybox", 0)?;

    // let cube_mesh = opengl::Mesh::create_and_bind(
    //     &[
    //         [0.0f32, 0.0, 0.0],
    //         [10.0, 0.0, 0.0],
    //         [-10.0, 0.0, 0.0],
    //         [0.0, 10.0, 0.0],
    //         [0.0, -10.0, 0.0],
    //         [0.0, 0.0, 10.0],
    //         [0.0, 0.0, -10.0],
    //     ],
    //     &[opengl::BufferAttribute {
    //         size: opengl_sys::VertexAttributeSize::Triple,
    //         data_type: opengl_sys::DataType::F32,
    //     }],
    //     None,
    //     opengl_sys::DrawMode::Points,
    // )?;

    #[rustfmt::skip]
    let vertices = [
        [-1.0f32, 1.0, -1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, -1.0],
        [1.0, -1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, -1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
    ];

    let skybox_mesh = opengl::Mesh::create_and_bind(
        &vertices,
        &[opengl::BufferAttribute {
            size: opengl_sys::VertexAttributeSize::Triple,
            data_type: opengl_sys::DataType::F32,
        }],
        None,
        opengl_sys::DrawMode::Points,
    )?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    // let model_location = shader_program.locate_uniform("model")?;
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

        let camera_view = camera.calculate_view();
        let camera_projection = camera.calculate_projection(window_size);

        shader_program.enable().unwrap();
        set_mat4(view_location, &camera_view);
        set_mat4(projection_location, &camera_projection);

        // opengl_sys::set_depth_func(opengl_sys::DepthFunc::LessEqual);
        skybox_shader_texture.draw().unwrap();
        skybox_mesh.draw().unwrap();
        // opengl_sys::set_depth_func(opengl_sys::DepthFunc::Less);
    })
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}
