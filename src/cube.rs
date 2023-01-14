use crate::assets::Vertex3D;

pub const VERTICES: [Vertex3D; 24] = [
    // Front
    Vertex3D {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [0.5, 0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
        position: [0.5, -0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Left
    Vertex3D {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Top
    Vertex3D {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
        position: [0.5, 0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    // Right
    Vertex3D {
        position: [0.5, -0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [0.5, 0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    // Bottom
    Vertex3D {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [0.5, -0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
        position: [0.5, -0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    // Rear
    Vertex3D {
        position: [-0.5, -0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex3D {
        position: [0.5, 0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex3D {
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
