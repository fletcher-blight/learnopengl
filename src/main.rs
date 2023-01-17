mod assets;
mod camera;
mod cube;
mod opengl;
mod shader;
mod skybox;
mod window;

extern crate anyhow;
extern crate env_logger;
extern crate log;
extern crate nalgebra_glm;
extern crate rand;

use assets::*;
use camera::Camera;
use nalgebra_glm as glm;
use rand::prelude::ThreadRng;
use rand::Rng;
use shader::*;
use window::*;

#[derive(Default)]
struct CameraControls {
    left: f32,
    right: f32,
    forward: f32,
    backward: f32,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut rng = rand::thread_rng();

    log::info!("Window initialisation ...");
    let window = Window::new("Rust Renderer", 1920, 1080)?;
    log::info!("Window initialisation ... complete");

    log::info!("Compiling shaders ...");

    let cube_shader_program = ShaderProgram::new(&[
        Shader::new(
            include_str!("../assets/shaders/cube.vert"),
            ShaderType::Vertex,
        )?,
        Shader::new(
            include_str!("../assets/shaders/cube.frag"),
            ShaderType::Fragment,
        )?,
    ])?;

    let skybox_shader_program = ShaderProgram::new(&[
        Shader::new(
            include_str!("../assets/shaders/skybox.vert"),
            ShaderType::Vertex,
        )?,
        Shader::new(
            include_str!("../assets/shaders/skybox.frag"),
            ShaderType::Fragment,
        )?,
    ])?;

    log::info!("Compiling shaders ... complete");

    log::info!("Loading assets ...");

    let asset_dir = std::env::current_dir()?.join("assets");
    let image_dir = asset_dir.join("images");
    let skybox_dir = asset_dir.join("skyboxes");

    let splatoon_texture = TextureImage2D::load_from_file(&image_dir.join("splatoon-face.jpeg"))?;
    let ship_c_texture = TextureImage2D::load_from_file(&image_dir.join("ship_C.png"))?;
    let skybox_texture = TextureCubeMap::load_from_file(&CubeMap {
        right: skybox_dir.join("right.jpg"),
        left: skybox_dir.join("left.jpg"),
        top: skybox_dir.join("top.jpg"),
        bottom: skybox_dir.join("bottom.jpg"),
        back: skybox_dir.join("back.jpg"),
        front: skybox_dir.join("front.jpg"),
    })?;

    let splatoon_shader_texture =
        ShaderProgramTexture::new(&splatoon_texture, &cube_shader_program, "tex1", 0)?;
    let ship_c_shader_texture =
        ShaderProgramTexture::new(&ship_c_texture, &cube_shader_program, "tex2", 1)?;
    let skybox_shader_texture =
        ShaderProgramTexture::new(&skybox_texture, &skybox_shader_program, "skybox", 0)?;

    let cube_mesh = Mesh::create_and_bind(
        &cube::VERTICES,
        &cube::ATTRIBUTES,
        Some(&cube::INDICES),
        DrawMode::Triangles,
    )?;
    let skybox_mesh = Mesh::create_and_bind(
        &skybox::VERTICES,
        &skybox::ATTRIBUTES,
        None,
        DrawMode::Triangles,
    )?;

    log::info!("Loading assets ... complete");

    log::info!("Initialising game logic ...");

    let cube_model_location = cube_shader_program.locate_uniform("model")?;
    let cube_view_location = cube_shader_program.locate_uniform("view")?;
    let cube_projection_location = cube_shader_program.locate_uniform("projection")?;

    let skybox_view_location = skybox_shader_program.locate_uniform("view")?;
    let skybox_projection_location = skybox_shader_program.locate_uniform("projection")?;

    const CAMERA_ACCELERATION: f32 = 100.0;
    const CAMERA_DRAG: f32 = 0.98;
    let mut camera = Camera::new();
    let mut camera_controls = CameraControls::default();
    let mut play = true;

    let cubes: Vec<_> = create_random_vectors(1000, &mut rng)
        .into_iter()
        .zip(create_random_vectors(1000, &mut rng).into_iter())
        .zip(create_random_vectors(1000, &mut rng).into_iter())
        .map(|((position, orbit), rotation)| (position, orbit, rotation))
        .collect();

    log::info!("Initialising game logic ... complete");

    log::info!("Game loop");

    let start_instant = std::time::Instant::now();
    let mut last_frame_instant = start_instant;
    let mut current_frame_instant = start_instant;

    window.run(|window_size, events| {
        current_frame_instant = std::time::Instant::now();
        let seconds_since_last_frame = current_frame_instant
            .duration_since(last_frame_instant)
            .as_secs_f32();
        let total_passed_seconds = if play {
            current_frame_instant
                .duration_since(start_instant)
                .as_secs_f32()
        } else {
            0.0
        };

        for event in events {
            match event {
                Event::KeyUp(Keycode::W) => camera_controls.forward = 0.0,
                Event::KeyUp(Keycode::S) => camera_controls.backward = 0.0,
                Event::KeyUp(Keycode::A) => camera_controls.left = 0.0,
                Event::KeyUp(Keycode::D) => camera_controls.right = 0.0,
                Event::KeyUp(_) => {}
                Event::KeyDown(Keycode::W) => camera_controls.forward = CAMERA_ACCELERATION,
                Event::KeyDown(Keycode::S) => camera_controls.backward = CAMERA_ACCELERATION,
                Event::KeyDown(Keycode::A) => camera_controls.left = CAMERA_ACCELERATION,
                Event::KeyDown(Keycode::D) => camera_controls.right = CAMERA_ACCELERATION,
                Event::KeyDown(Keycode::Space) => play = !play,
                Event::KeyDown(_) => {}
                Event::MousePosition(xrel, yrel) => {
                    camera.move_orientation(*xrel * 0.05, *yrel * 0.05)
                }
                Event::MouseScroll(offset) => camera.zoom(*offset * 10.0, seconds_since_last_frame),
            }
        }
        camera.move_position(
            camera_controls.calculate_force(),
            CAMERA_DRAG,
            seconds_since_last_frame,
        );

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let camera_view = camera.calculate_view();
        let camera_projection = camera.calculate_projection(window_size);

        cube_shader_program.enable().unwrap();
        splatoon_shader_texture.draw().unwrap();
        ship_c_shader_texture.draw().unwrap();
        set_uniform_mat4(cube_view_location, &camera_view).unwrap();
        set_uniform_mat4(cube_projection_location, &camera_projection).unwrap();
        for (position, orbit, rotation) in &cubes {
            set_uniform_mat4(
                cube_model_location,
                &calculate_model(total_passed_seconds, position, orbit, rotation),
            )
            .unwrap();
            cube_mesh.draw().unwrap();
        }

        unsafe { gl::DepthFunc(gl::LEQUAL) };
        skybox_shader_texture.draw().unwrap();
        skybox_shader_program.enable().unwrap();
        set_uniform_mat4(
            skybox_view_location,
            &glm::mat3_to_mat4(&glm::mat4_to_mat3(&camera_view)),
        )
        .unwrap();
        set_uniform_mat4(skybox_projection_location, &camera_projection).unwrap();
        skybox_mesh.draw().unwrap();
        unsafe { gl::DepthFunc(gl::LESS) };

        last_frame_instant = current_frame_instant;
    })
}

fn set_uniform_mat4(location: opengl::UniformLocation, mat: &glm::Mat4) -> anyhow::Result<()> {
    opengl::set_uniform_mat4(location, false, glm::value_ptr(mat))?;
    Ok(())
}

fn calculate_model(
    total_passed_seconds: f32,
    position: &glm::Vec3,
    orbit: &glm::Vec3,
    rotation: &glm::Vec3,
) -> glm::Mat4 {
    glm::rotate(
        &glm::translate(
            &glm::rotate(
                &glm::one(),
                (total_passed_seconds * 20.0).to_radians(),
                &orbit,
            ),
            position,
        ),
        (total_passed_seconds * 120.0).to_radians(),
        rotation,
    )
}

impl CameraControls {
    fn calculate_force(&self) -> glm::Vec3 {
        glm::vec3(self.right - self.left, 0.0, self.forward - self.backward)
    }
}

fn create_random_vectors(n: usize, rng: &mut ThreadRng) -> Vec<glm::Vec3> {
    std::iter::repeat_with(|| {
        glm::vec3(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        )
    })
    .take(n)
    .collect()
}
