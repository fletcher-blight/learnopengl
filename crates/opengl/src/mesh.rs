pub use opengl_sys::{DataType, DrawMode};

use crate::buffer::*;
use opengl_sys::BufferTarget;

#[derive(Clone)]
pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    instance_buffer: Option<Buffer>,
}

impl<const N: usize> TryFrom<&[[f32; N]]> for Mesh {
    type Error = anyhow::Error;
    fn try_from(vertices: &[[f32; N]]) -> Result<Self, Self::Error> {
        let attribute_size = match N {
            2 => BufferAttributeSize::Double,
            3 => BufferAttributeSize::Triple,
            _ => anyhow::bail!("Invalid Buffer Size"),
        };

        let vertex_array = VertexArray::new();
        let mut vertex_buffer = Buffer::new(BufferTarget::Array);

        vertex_array.bind()?;
        vertex_buffer.bind(vertices, &[(0, attribute_size).into()])?;
        opengl_sys::bind_vertex_array(0)?;

        Ok(Mesh {
            vertex_array,
            vertex_buffer,
            index_buffer: None,
            instance_buffer: None,
        })
    }
}

impl Mesh {
    pub fn draw(&self, draw_mode: DrawMode) -> anyhow::Result<()> {
        self.vertex_array.bind()?;

        if let Some(ibo) = &self.instance_buffer {
            if let Some(ebo) = &self.index_buffer {
                opengl_sys::draw_elements_instanced(
                    draw_mode,
                    ebo.size,
                    DataType::U32,
                    ibo.size as _,
                )?;
            } else {
                opengl_sys::draw_arrays_instanced(
                    draw_mode,
                    0,
                    self.vertex_buffer.size,
                    ibo.size as _,
                )?;
            }
        } else {
            if let Some(ebo) = &self.index_buffer {
                opengl_sys::draw_elements(draw_mode, ebo.size, DataType::U32)?;
            } else {
                opengl_sys::draw_arrays(draw_mode, 0, self.vertex_buffer.size)?;
            }
        }

        opengl_sys::bind_vertex_array(0)?;
        Ok(())
    }
}
