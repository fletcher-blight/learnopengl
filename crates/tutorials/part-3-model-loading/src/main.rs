use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let window = winman::Window::new("21-model-loading", 1920, 1080)?;

    let shader_program = opengl::ShaderProgram::new(&[
        opengl::Shader::new(include_str!("shader.vert"), opengl::ShaderType::Vertex)?,
        opengl::Shader::new(include_str!("shader.frag"), opengl::ShaderType::Fragment)?,
    ])?;
    shader_program.enable()?;

    let asset_dir = std::env::current_dir()?.join("assets");
    let objects_dir = asset_dir.join("objects");
    let backpack_dir = objects_dir.join("backpack");

    let model = Model::load_from_file(backpack_dir.join("backpack.obj"))?;

    let mut camera = camera::Camera::new();
    let mut camera_controls = camera::Controls::default();

    camera.set_position(&[0.0, 0.0, 3.0]);

    let model_location = shader_program.locate_uniform("model")?;
    let view_location = shader_program.locate_uniform("view")?;
    let projection_location = shader_program.locate_uniform("projection")?;
    let view_pos_location = shader_program.locate_uniform("view_pos")?;
    let material_shininess_location = shader_program.locate_uniform("material.shininess")?;
    let light_direction_location = shader_program.locate_uniform("light.direction")?;
    let light_ambient_location = shader_program.locate_uniform("light.ambient")?;
    let light_diffuse_location = shader_program.locate_uniform("light.diffuse")?;
    let light_specular_location = shader_program.locate_uniform("light.specular")?;

    opengl_sys::set_uniform_f32(material_shininess_location, 32.0)?;
    opengl_sys::set_uniform_vec3(light_direction_location, &[-0.2, -1.0, -0.3]).unwrap();
    opengl_sys::set_uniform_vec3(light_ambient_location, &[0.2, 0.2, 0.2])?;
    opengl_sys::set_uniform_vec3(light_diffuse_location, &[0.5, 0.5, 0.5])?;
    opengl_sys::set_uniform_vec3(light_specular_location, &[1.0, 1.0, 1.0])?;

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
        opengl_sys::set_uniform_vec3(view_pos_location, &camera.get_position()).unwrap();

        model.draw(&shader_program).unwrap();
    })
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
