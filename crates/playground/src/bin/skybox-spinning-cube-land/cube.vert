#version 330 core

in vec3 position;
out vec2 tex_coords;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
    switch (gl_VertexID % 6) {
        case 0: case 3: tex_coords = vec2(0.0, 0.0); break;
        case 1:         tex_coords = vec2(0.0, 1.0); break;
        case 2: case 4: tex_coords = vec2(1.0, 1.0); break;
        case 5:         tex_coords = vec2(1.0, 0.0); break;
    }
    gl_Position = projection * view * model * vec4(position, 1.0);
}