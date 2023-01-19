#version 330 core

layout (location=0) in vec3 position;

out vec2 v_tex_coords;
out vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    // texture coordinates
    switch (gl_VertexID % 6) {
        case 0: case 3: v_tex_coords = vec2(0.0, 0.0); break;
        case 1:         v_tex_coords = vec2(0.0, 1.0); break;
        case 2: case 4: v_tex_coords = vec2(1.0, 1.0); break;
        case 5:         v_tex_coords = vec2(1.0, 0.0); break;
    }

    // normals
    switch (gl_VertexID / 6) {
        case 0: v_normal = vec3(0.0, 0.0, -1.0); break;
        case 1: v_normal = vec3(-1.0, 0.0, 0.0); break;
        case 2: v_normal = vec3(0.0, 1.0, 0.0); break;
        case 3: v_normal = vec3(1.0, 0.0, 0.0); break;
        case 4: v_normal = vec3(0.0, -1.0, 0.0); break;
        case 5: v_normal = vec3(0.0, 0.0, 1.0); break;
    }

    gl_Position = projection * view * model * vec4(position, 1.0);
}