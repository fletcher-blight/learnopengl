#version 330 core

layout (location=0) in vec2 position;

out vec2 v_tex_coords;

void main() {
    switch (gl_VertexID) {
        case 0: case 3: v_tex_coords = vec2(0.0, 0.0); break;
        case 1:         v_tex_coords = vec2(0.0, 1.0); break;
        case 2: case 4: v_tex_coords = vec2(1.0, 1.0); break;
        case 5:         v_tex_coords = vec2(1.0, 0.0); break;
    }
    gl_Position = vec4(position, 0.0, 1.0);
}