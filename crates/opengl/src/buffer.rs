pub use opengl_sys::VertexAttributeSize as BufferAttributeSize;
pub use opengl_sys::{BufferTarget, DataType};

#[derive(Clone)]
pub struct VertexArray {
    id: opengl_sys::VertexArrayID,
}

impl VertexArray {
    pub fn new() -> Self {
        Self {
            id: opengl_sys::create_vertex_array(),
        }
    }

    pub fn bind(&self) -> anyhow::Result<()> {
        opengl_sys::bind_vertex_array(self.id)?;
        Ok(())
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        opengl_sys::delete_vertex_array(self.id).expect("Failed to delete vertex array");
    }
}

#[derive(Clone)]
pub struct Buffer {
    id: opengl_sys::BufferID,
    target: BufferTarget,
    pub(crate) size: u64,
}

#[derive(Copy, Clone)]
pub struct BufferAttribute {
    pub size: BufferAttributeSize,
    pub data_type: DataType,
}

impl Buffer {
    pub fn new(target: BufferTarget) -> Self {
        Self {
            id: opengl_sys::create_buffer(),
            target,
            size: 0,
        }
    }

    pub fn bind<Vertex>(
        &mut self,
        vertices: &[Vertex],
        attribute_layout: &[BufferAttribute],
    ) -> anyhow::Result<()> {
        opengl_sys::bind_buffer(self.id, self.target)?;
        opengl_sys::set_buffer_data(self.target, opengl_sys::BufferUsage::StaticDraw, vertices)?;
        self.size = vertices.len() as _;

        let stride = attribute_layout
            .iter()
            .map(|attribute| attribute.size.as_value() * attribute.data_type.num_bytes())
            .sum();

        let mut offset = 0;
        for (index, attribute) in attribute_layout.iter().enumerate() {
            opengl_sys::enable_vertex_attribute_array(index as _)?;
            opengl_sys::vertex_attribute_pointer(
                index as _,
                attribute.size,
                attribute.data_type,
                false,
                stride,
                offset,
            )?;

            offset += attribute.size.as_value() * attribute.data_type.num_bytes();
        }

        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        opengl_sys::delete_buffer(self.id).expect("Failed to delete buffer");
    }
}
