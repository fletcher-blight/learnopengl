use nalgebra_glm as glm;
use rand::Rng;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("Playground: Skybox Spinning Cube Land", 1920, 1080)?;

    let cube_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("cube.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("cube.frag"), opengl::ShaderType::Fragment)?,
    ])?;
    let skybox_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("skybox.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("skybox.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let images_dir = asset_dir.join("images");
    let skybox_dir = asset_dir.join("skyboxes");

    let cube_texture =
        opengl::TextureImage2D::load_from_file(&images_dir.join("splatoon-face.jpeg"))?;
    let skybox_texture = opengl::TextureCubeMap::load_from_file(&opengl::CubeMap {
        right: skybox_dir.join("right.jpg"),
        left: skybox_dir.join("left.jpg"),
        top: skybox_dir.join("top.jpg"),
        bottom: skybox_dir.join("bottom.jpg"),
        back: skybox_dir.join("back.jpg"),
        front: skybox_dir.join("front.jpg"),
    })?;

    let cube_shader_texture =
        opengl::ShaderProgramTexture::new(&cube_texture, &cube_shader_program, "tex", 0)?;
    let skybox_shader_texture =
        opengl::ShaderProgramTexture::new(&skybox_texture, &skybox_shader_program, "skybox", 0)?;

    #[rustfmt::skip]
    let cube_vertices = [
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

    let mut rng = rand::thread_rng();
    let cube_offests: Vec<_> = std::iter::repeat_with(|| create_random_vec(&mut rng))
        .take(500000)
        .collect();

    #[rustfmt::skip]
    let skybox_vertices = [
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

    let skybox_mesh: opengl::Mesh = skybox_vertices.as_slice().try_into()?;
    let cube_mesh = opengl::Mesh::new(
        &cube_vertices,
        &[(1, opengl::BufferAttributeSize::Triple).into()],
        None as Option<&[()]>,
        Some((
            cube_offests.as_slice(),
            &[opengl::BufferAttribute {
                index: 2,
                size: opengl::BufferAttributeSize::Triple,
                data_type: opengl::DataType::F32,
                divisor: 1,
            }],
        )),
    )?;

    let cube_model_location = cube_shader_program.locate_uniform("model")?;
    let cube_view_location = cube_shader_program.locate_uniform("view")?;
    let cube_projection_location = cube_shader_program.locate_uniform("projection")?;

    let skybox_view_location = skybox_shader_program.locate_uniform("view")?;
    let skybox_projection_location = skybox_shader_program.locate_uniform("projection")?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    window.run(
        |window_size, (total_seconds_passed, seconds_since_last_frame), events| {
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

            cube_shader_program.enable().unwrap();
            set_mat4(
                cube_model_location,
                &glm::rotate(
                    &glm::one(),
                    (10.0 * total_seconds_passed).to_radians(),
                    &glm::vec3(0.0, 1.0, 0.0),
                ),
            );
            set_mat4(cube_view_location, &camera_view);
            set_mat4(cube_projection_location, &camera_projection);
            cube_shader_texture.draw().unwrap();
            cube_mesh.draw(opengl::DrawMode::Triangles).unwrap();

            skybox_shader_program.enable().unwrap();
            set_mat4(skybox_view_location, &camera_view);
            set_mat4(skybox_projection_location, &camera_projection);
            skybox_shader_texture.draw().unwrap();
            opengl_sys::set_depth_func(opengl_sys::DepthFunc::LessEqual);
            skybox_mesh.draw(opengl::DrawMode::Triangles).unwrap();
            opengl_sys::set_depth_func(opengl_sys::DepthFunc::Less);

            println!("FPS: {}", 1.0 / seconds_since_last_frame);
        },
    )
}

fn set_mat4(location: opengl::UniformLocation, mat4: &glm::Mat4) {
    opengl_sys::set_uniform_mat4(location, false, glm::value_ptr(mat4)).unwrap();
}

fn create_random_vec(rng: &mut rand::rngs::ThreadRng) -> [f32; 3] {
    [
        rng.gen_range(-100.0..100.0),
        rng.gen_range(-100.0..100.0),
        rng.gen_range(-100.0..100.0),
    ]
}
