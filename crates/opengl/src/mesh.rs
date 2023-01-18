pub use opengl_sys::{ DataType, DrawMode };

use opengl_sys::BufferTarget;
use crate::buffer::*;

#[derive(Clone)]
pub struct Mesh {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Option<Buffer>,
    draw_mode: DrawMode,
}

impl Mesh {
    pub fn new(vao: VertexArray, vbo: Buffer, ebo: Option<Buffer>, draw_mode: DrawMode) -> Self {
        Self {
            vao,
            vbo,
            ebo,
            draw_mode,
        }
    }

    pub fn create_and_bind<Vertex>(
        vertices: &[Vertex],
        attribute_layout: &[BufferAttribute],
        indices: Option<&[u32]>,
        draw_mode: DrawMode,
    ) -> anyhow::Result<Self> {
        let vao = VertexArray::new();
        let mut vbo = Buffer::new(BufferTarget::Array);

        vao.bind()?;
        vbo.bind(vertices, attribute_layout)?;

        let ebo = if let Some(indices) = indices {
            let mut buffer = Buffer::new(BufferTarget::ElementArray);
            buffer.bind(indices, Default::default())?;
            Some(buffer)
        } else {
            None
        };

        opengl_sys::bind_vertex_array(0)?;
        Ok(Self {
            vao,
            vbo,
            ebo,
            draw_mode,
        })
    }

    pub fn draw(&self) -> anyhow::Result<()> {
        self.vao.bind()?;

        if let Some(ebo) = &self.ebo {
            opengl_sys::draw_elements(self.draw_mode, ebo.size, DataType::U32)?;
        } else {
            opengl_sys::draw_arrays(self.draw_mode, 0, self.vbo.size)?;
        }

        Ok(())
    }
}
