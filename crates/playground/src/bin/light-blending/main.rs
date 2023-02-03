use nalgebra_glm as glm;
use opengl_sys::DrawMode;

const SCREEN_WIDTH: u32 = 3840;
const SCREEN_HEIGHT: u32 = 2160;

struct DirectionalProgram {
    shader: opengl::ShaderProgram,
    model: opengl::UniformLocation,
    view: opengl::UniformLocation,
    projection: opengl::UniformLocation,
    view_pos: opengl::UniformLocation,
    material_shininess: opengl::UniformLocation,
    direction: opengl::UniformLocation,
    ambient: opengl::UniformLocation,
    diffuse: opengl::UniformLocation,
    specular: opengl::UniformLocation,
}

impl DirectionalProgram {
    fn new(vertex_src: &str, fragment_src: &str) -> anyhow::Result<Self> {
        let shader = opengl::ShaderProgram::new(&[
            opengl::Shader::new(vertex_src, opengl::ShaderType::Vertex)?,
            opengl::Shader::new(fragment_src, opengl::ShaderType::Fragment)?,
        ])?;

        shader.enable()?;
        let model = shader.locate_uniform("model")?;
        let view = shader.locate_uniform("view")?;
        let projection = shader.locate_uniform("projection")?;
        let view_pos = shader.locate_uniform("view_pos")?;
        let material_shininess = shader.locate_uniform("material.shininess")?;
        let direction = shader.locate_uniform("light.direction")?;
        let ambient = shader.locate_uniform("light.ambient")?;
        let diffuse = shader.locate_uniform("light.diffuse")?;
        let specular = shader.locate_uniform("light.specular")?;

        Ok(DirectionalProgram {
            shader,
            model,
            view,
            projection,
            view_pos,
            material_shininess,
            direction,
            ambient,
            diffuse,
            specular,
        })
    }

    fn set_light(
        &self,
        material_shininess: f32,
        direction: &[f32; 3],
        ambient_factor: f32,
        diffuse_factor: f32,
        specular_factor: f32,
    ) -> anyhow::Result<()> {
        opengl_sys::set_uniform_f32(self.material_shininess, material_shininess)?;
        opengl_sys::set_uniform_vec3(self.direction, direction)?;
        opengl_sys::set_uniform_vec3(
            self.ambient,
            &[ambient_factor, ambient_factor, ambient_factor],
        )?;
        opengl_sys::set_uniform_vec3(
            self.diffuse,
            &[diffuse_factor, diffuse_factor, diffuse_factor],
        )?;
        opengl_sys::set_uniform_vec3(
            self.specular,
            &[specular_factor, specular_factor, specular_factor],
        )?;
        Ok(())
    }

    fn set_mvp(
        &self,
        model: &glm::Mat4,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        view_position: &[f32; 3],
    ) -> anyhow::Result<()> {
        self.shader.enable()?;
        opengl_sys::set_uniform_mat4(self.model, false, glm::value_ptr::<f32, 4, 4>(model))?;
        opengl_sys::set_uniform_mat4(self.view, false, glm::value_ptr(view))?;
        opengl_sys::set_uniform_mat4(self.projection, false, glm::value_ptr(projection))?;
        opengl_sys::set_uniform_vec3(self.view_pos, view_position)?;
        Ok(())
    }
}

struct PointLightProgram {
    shader: opengl::ShaderProgram,
    model: opengl::UniformLocation,
    view: opengl::UniformLocation,
    projection: opengl::UniformLocation,
    view_pos: opengl::UniformLocation,
    material_shininess: opengl::UniformLocation,
    position: opengl::UniformLocation,
    ambient: opengl::UniformLocation,
    diffuse: opengl::UniformLocation,
    specular: opengl::UniformLocation,
    attenuation_linear: opengl::UniformLocation,
    attenuation_quadratic: opengl::UniformLocation,
}

impl PointLightProgram {
    fn new(vertex_src: &str, fragment_src: &str) -> anyhow::Result<Self> {
        let shader = opengl::ShaderProgram::new(&[
            opengl::Shader::new(vertex_src, opengl::ShaderType::Vertex)?,
            opengl::Shader::new(fragment_src, opengl::ShaderType::Fragment)?,
        ])?;

        shader.enable()?;
        let model = shader.locate_uniform("model")?;
        let view = shader.locate_uniform("view")?;
        let projection = shader.locate_uniform("projection")?;
        let view_pos = shader.locate_uniform("view_pos")?;
        let material_shininess = shader.locate_uniform("material.shininess")?;
        let position = shader.locate_uniform("light.position")?;
        let ambient = shader.locate_uniform("light.ambient")?;
        let diffuse = shader.locate_uniform("light.diffuse")?;
        let specular = shader.locate_uniform("light.specular")?;
        let attenuation_linear = shader.locate_uniform("light.attenuation_linear")?;
        let attenuation_quadratic = shader.locate_uniform("light.attenuation_quadratic")?;

        Ok(PointLightProgram {
            shader,
            model,
            view,
            projection,
            view_pos,
            material_shininess,
            position,
            ambient,
            diffuse,
            specular,
            attenuation_linear,
            attenuation_quadratic,
        })
    }

    fn set_light(
        &self,
        material_shininess: f32,
        ambient_factor: f32,
        diffuse_factor: f32,
        specular_factor: f32,
        attenuation_linear: f32,
        attenuation_quadratic: f32,
    ) -> anyhow::Result<()> {
        opengl_sys::set_uniform_f32(self.material_shininess, material_shininess)?;
        opengl_sys::set_uniform_vec3(
            self.ambient,
            &[ambient_factor, ambient_factor, ambient_factor],
        )?;
        opengl_sys::set_uniform_vec3(
            self.diffuse,
            &[diffuse_factor, diffuse_factor, diffuse_factor],
        )?;
        opengl_sys::set_uniform_vec3(
            self.specular,
            &[specular_factor, specular_factor, specular_factor],
        )?;
        opengl_sys::set_uniform_f32(self.attenuation_linear, attenuation_linear)?;
        opengl_sys::set_uniform_f32(self.attenuation_quadratic, attenuation_quadratic)?;
        Ok(())
    }

    fn set_mvp(
        &self,
        model: &glm::Mat4,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        view_position: &[f32; 3],
        light_position: &[f32; 3],
    ) -> anyhow::Result<()> {
        self.shader.enable()?;
        opengl_sys::set_uniform_mat4(self.model, false, glm::value_ptr::<f32, 4, 4>(model))?;
        opengl_sys::set_uniform_mat4(self.view, false, glm::value_ptr(view))?;
        opengl_sys::set_uniform_mat4(self.projection, false, glm::value_ptr(projection))?;
        opengl_sys::set_uniform_vec3(self.view_pos, view_position)?;
        opengl_sys::set_uniform_vec3(self.position, light_position)?;
        Ok(())
    }
}

struct FrameBuffer {
    frame_buffer_id: opengl_sys::FrameBufferID,
    texture_image_id: opengl_sys::TextureID,
    index: u32,
}

impl FrameBuffer {
    fn new(shader: &opengl::ShaderProgram, index: u32) -> anyhow::Result<Self> {
        let frame_buffer_id = opengl_sys::create_frame_buffer();
        opengl_sys::bind_frame_buffer(frame_buffer_id, opengl_sys::FrameBufferTarget::All)?;

        let texture_image_id = opengl_sys::create_texture();
        opengl_sys::bind_texture(texture_image_id, opengl_sys::TextureTarget::Image2D)?;
        opengl_sys::load_texture_image2d(
            opengl_sys::TextureTarget::Image2D,
            0,
            opengl_sys::TextureFormat::RGB,
            SCREEN_WIDTH as _,
            SCREEN_HEIGHT as _,
            opengl_sys::TextureFormat::RGB,
            opengl_sys::DataType::U32,
            None as Option<&[()]>,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::MinFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;
        opengl_sys::set_texture_parameter_value(
            opengl_sys::TextureTarget::Image2D,
            opengl_sys::TextureParameterName::MagFilter,
            opengl_sys::TextureParameterValue::Linear,
        )?;

        let render_buffer_id = opengl_sys::create_render_buffer();
        opengl_sys::bind_render_buffer(render_buffer_id)?;
        opengl_sys::render_buffer_storage(
            opengl_sys::RenderBufferStorageFormat::Depth24Stencil8,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )?;
        opengl_sys::frame_buffer_render_buffer(
            opengl_sys::FrameBufferTarget::All,
            opengl_sys::FrameBufferAttachment::DepthStencil,
            render_buffer_id,
        )?;

        shader.enable()?;
        let location = shader.locate_uniform(format!("frame_texture[{index}]").as_str())?;
        opengl_sys::set_uniform_i32(location, index as _)?;

        Ok(FrameBuffer {
            frame_buffer_id,
            texture_image_id,
            index,
        })
    }

    fn bind(&self) -> anyhow::Result<()> {
        opengl_sys::bind_frame_buffer(self.frame_buffer_id, opengl_sys::FrameBufferTarget::All)?;
        opengl_sys::enable(opengl_sys::Feature::DepthTest)?;

        opengl_sys::clear_colour(0.0, 0.0, 0.0, 1.0)?;
        opengl_sys::clear(opengl_sys::BufferBit::Colour)?;
        opengl_sys::clear(opengl_sys::BufferBit::Depth)?;

        opengl_sys::frame_buffer_texture_2d(
            opengl_sys::FrameBufferTarget::All,
            opengl_sys::FrameBufferAttachment::Colour(0),
            opengl_sys::TextureTarget::Image2D,
            self.texture_image_id,
            0,
        )?;

        Ok(())
    }

    fn active_texture(&self) -> anyhow::Result<()> {
        opengl_sys::active_texture(self.index)?;
        opengl_sys::bind_texture(self.texture_image_id, opengl_sys::TextureTarget::Image2D)?;
        Ok(())
    }

    fn enable_display_buffer() -> anyhow::Result<()> {
        opengl_sys::bind_frame_buffer(0, opengl_sys::FrameBufferTarget::All)?;
        opengl_sys::disable(opengl_sys::Feature::DepthTest)?;

        opengl_sys::clear_colour(0.0, 0.0, 0.0, 1.0)?;
        opengl_sys::clear(opengl_sys::BufferBit::Colour)?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("21-model-loading", SCREEN_WIDTH, SCREEN_HEIGHT)?;

    let quad_mesh: opengl::Mesh = ([
        [-1.0, -1.0],
        [-1.0, 1.0],
        [1.0, 1.0],
        [-1.0, -1.0],
        [1.0, 1.0],
        [1.0, -1.0],
    ]
    .as_slice())
    .try_into()?;

    let frame_shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("frame.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("frame.frag"), opengl::ShaderType::Fragment)?,
    ])?;

    let directional_top_light = DirectionalProgram::new(
        include_str!("shader.vert"),
        include_str!("directional.frag"),
    )?;
    directional_top_light.set_light(32.0, &[0.0, -1.0, 0.0], 0.2, 0.5, 1.0)?;
    let directional_top_light_frame_buffer = FrameBuffer::new(&frame_shader_program, 0)?;

    let directional_bottom_light = DirectionalProgram::new(
        include_str!("shader.vert"),
        include_str!("directional.frag"),
    )?;
    directional_bottom_light.set_light(32.0, &[0.0, 1.0, 0.0], 0.2, 0.5, 1.0)?;
    let directional_bottom_light_frame_buffer = FrameBuffer::new(&frame_shader_program, 1)?;

    let point_light1 = PointLightProgram::new(
        include_str!("shader.vert"),
        include_str!("point_light.frag"),
    )?;
    point_light1.set_light(32.0, 0.2, 1.0, 1.0, 0.12, 0.032)?;
    let point_light1_frame_buffer = FrameBuffer::new(&frame_shader_program, 2)?;

    let point_light2 = PointLightProgram::new(
        include_str!("shader.vert"),
        include_str!("point_light.frag"),
    )?;
    point_light2.set_light(32.0, 0.2, 1.0, 1.0, 0.12, 0.032)?;
    let point_light2_frame_buffer = FrameBuffer::new(&frame_shader_program, 3)?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let objects_dir = asset_dir.join("objects");
    let backpack_dir = objects_dir.join("backpack");
    let model = Model::load_from_file(backpack_dir.join("backpack.obj"))?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();
    camera.set_position(&[0.0, 0.0, 3.0]);

    window.run(
        |window_size, (total_passed_seconds, seconds_since_last_frame), events| {
            camera::process_events(
                &mut camera,
                &mut camera_controls,
                70.0,
                0.97,
                seconds_since_last_frame,
                events,
            );

            directional_top_light_frame_buffer.bind().unwrap();
            directional_top_light
                .set_mvp(
                    &glm::one(),
                    &camera.calculate_view(),
                    &camera.calculate_projection(window_size),
                    &camera.get_position(),
                )
                .unwrap();
            model.draw(&directional_top_light.shader).unwrap();

            directional_bottom_light_frame_buffer.bind().unwrap();
            directional_bottom_light
                .set_mvp(
                    &glm::one(),
                    &camera.calculate_view(),
                    &camera.calculate_projection(window_size),
                    &camera.get_position(),
                )
                .unwrap();
            model.draw(&directional_bottom_light.shader).unwrap();

            let spin_angle1 = (total_passed_seconds * 50.0).to_radians();
            let spin_angle2 = (total_passed_seconds * 75.0).to_radians();

            point_light1_frame_buffer.bind().unwrap();
            point_light1
                .set_mvp(
                    &glm::one(),
                    &camera.calculate_view(),
                    &camera.calculate_projection(window_size),
                    &camera.get_position(),
                    &[5.0 * spin_angle1.cos(), 0.0, 5.0 * spin_angle1.sin()],
                )
                .unwrap();
            model.draw(&point_light1.shader).unwrap();

            point_light2_frame_buffer.bind().unwrap();
            point_light2
                .set_mvp(
                    &glm::one(),
                    &camera.calculate_view(),
                    &camera.calculate_projection(window_size),
                    &camera.get_position(),
                    &[0.0, 5.0 * spin_angle2.cos(), 5.0 * spin_angle2.sin()],
                )
                .unwrap();
            model.draw(&point_light2.shader).unwrap();

            FrameBuffer::enable_display_buffer().unwrap();
            frame_shader_program.enable().unwrap();
            directional_top_light_frame_buffer.active_texture().unwrap();
            directional_bottom_light_frame_buffer
                .active_texture()
                .unwrap();
            point_light1_frame_buffer.active_texture().unwrap();
            point_light2_frame_buffer.active_texture().unwrap();
            quad_mesh.draw(DrawMode::Triangles).unwrap();
        },
    )
}

struct TexturedMesh {
    diffuse: opengl::TextureImage2D,
    specular: opengl::TextureImage2D,
    meshes: Vec<opengl::Mesh>,
}

impl TexturedMesh {
    fn draw(&self, shader: &opengl::ShaderProgram) -> anyhow::Result<()> {
        let shader_diffuse_texture =
            opengl::ShaderProgramTexture::new(&self.diffuse, &shader, "material.diffuse", 0)?;
        let shader_specular_texture =
            opengl::ShaderProgramTexture::new(&self.specular, &shader, "material.specular", 1)?;

        shader_diffuse_texture.draw()?;
        shader_specular_texture.draw()?;

        for mesh in &self.meshes {
            mesh.draw(opengl::DrawMode::Triangles)?;
        }

        Ok(())
    }
}

struct Model {
    textured_meshes: Vec<Option<TexturedMesh>>,
}

impl Model {
    fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let scene = russimp::scene::Scene::from_file(
            &path.as_ref().to_str().unwrap(),
            vec![
                russimp::scene::PostProcess::FlipUVs,
                russimp::scene::PostProcess::Triangulate,
            ],
        )?;

        let parent_path = path.as_ref().parent().unwrap();

        let mut textured_meshes: Vec<_> = Default::default();
        for material in scene.materials {
            let diffuse = Self::load_material(
                parent_path,
                &material.properties,
                russimp::material::TextureType::Diffuse,
            )?;

            let specular = Self::load_material(
                parent_path,
                &material.properties,
                russimp::material::TextureType::Specular,
            )?;

            if diffuse.is_none() || specular.is_none() {
                textured_meshes.push(None);
                continue;
            }

            textured_meshes.push(Some(TexturedMesh {
                diffuse: diffuse.unwrap(),
                specular: specular.unwrap(),
                meshes: Default::default(),
            }))
        }

        for mesh in &scene.meshes {
            let mut indices: Vec<u32> = Default::default();

            for face in &mesh.faces {
                indices.append(&mut face.0.clone());
            }

            let uv_chan = mesh.texture_coords[0].as_ref().unwrap();

            let vertices: Vec<_> = itertools::izip!(&mesh.vertices, &mesh.normals, uv_chan)
                .map(|(position, normal, uv)| {
                    [
                        position.x, position.y, position.z, normal.x, normal.y, normal.z, uv.x,
                        uv.y,
                    ]
                })
                .collect();

            let index = mesh.material_index as usize;
            if textured_meshes.len() <= index {
                anyhow::bail!(format!("invalid material index: {index}"));
            }

            let mesh = opengl::Mesh::new(
                &vertices,
                &[
                    (0, opengl::BufferAttributeSize::Triple).into(),
                    (1, opengl::BufferAttributeSize::Triple).into(),
                    (2, opengl::BufferAttributeSize::Double).into(),
                ],
                Some(&indices),
                None as Option<(&[()], &[opengl::BufferAttribute])>,
            )?;
            if let Some(textured_mesh) = &mut textured_meshes[index] {
                textured_mesh.meshes.push(mesh);
            }
        }

        Ok(Model { textured_meshes })
    }

    fn load_material<P: AsRef<std::path::Path>>(
        parent_path: P,
        properties: &[russimp::material::MaterialProperty],
        target: russimp::material::TextureType,
    ) -> anyhow::Result<Option<opengl::TextureImage2D>> {
        let property = properties
            .iter()
            .find(|property| property.key == "$tex.file" && property.semantic == target);

        if property.is_none() {
            return Ok(None);
        }

        let property = property.unwrap();

        let texture = if let russimp::material::PropertyTypeInfo::String(filename) = &property.data
        {
            Some(opengl::TextureImage2D::load_from_file(
                &parent_path.as_ref().join(filename),
            )?)
        } else {
            None
        };

        Ok(texture)
    }

    fn draw(&self, shader: &opengl::ShaderProgram) -> anyhow::Result<()> {
        for mesh in &self.textured_meshes {
            if let Some(mesh) = mesh {
                mesh.draw(&shader).unwrap();
            }
        }
        Ok(())
    }
}
