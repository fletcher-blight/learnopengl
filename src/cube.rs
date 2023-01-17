use crate::assets::{BufferAttribute, BufferAttributeSize};
use crate::opengl;

#[repr(C, packed)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

pub const ATTRIBUTES: [BufferAttribute; 2] = [
    BufferAttribute {
        data_type: opengl::DataType::F32,
        size: BufferAttributeSize::Triple,
    },
    BufferAttribute {
        data_type: opengl::DataType::F32,
        size: BufferAttributeSize::Double,
    },
];

pub const VERTICES: [Vertex; 24] = [
    // Front
    Vertex {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Left
    Vertex {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Top
    Vertex {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Right
    Vertex {
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    // Bottom
    Vertex {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    // Rear
    Vertex {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
];

#[rustfmt::skip]
pub const INDICES: [u32; 36] = [
    0, 1, 2,
    0, 2, 3,

    4, 5, 6,
    4, 6, 7,
    
    8, 9, 10,
    8, 10, 11,
    
    12, 13, 14,
    12, 14, 15,
    
    16, 17, 18,
    16, 18, 19,
    
    20, 21, 22,
    20, 22, 23,
];
