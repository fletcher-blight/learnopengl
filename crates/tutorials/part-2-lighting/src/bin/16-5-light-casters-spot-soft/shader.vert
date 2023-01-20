#version 330 core

layout (location=0) in vec3 position;

out vec3 normal;
out vec3 frag_word_position;
out vec2 tex_coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    switch (gl_VertexID % 6) {
        case 0: case 3: tex_coord = vec2(0.0, 0.0); break;
        case 1:         tex_coord = vec2(0.0, 1.0); break;
        case 2: case 4: tex_coord = vec2(1.0, 1.0); break;
        case 5:         tex_coord = vec2(1.0, 0.0); break;
    }

    mat3 normal_transform = mat3(transpose(inverse(model)));
    switch (gl_VertexID / 6) {
        case 0: normal = normal_transform * vec3(0.0, 0.0, 1.0); break;
        case 1: normal = normal_transform * vec3(-1.0, 0.0, 0.0); break;
        case 2: normal = normal_transform * vec3(0.0, 1.0, 0.0); break;
        case 3: normal = normal_transform * vec3(1.0, 0.0, 0.0); break;
        case 4: normal = normal_transform * vec3(0.0, -1.0, 0.0); break;
        case 5: normal = normal_transform * vec3(0.0, 0.0, -1.0); break;
    }

    frag_word_position = vec3(model * vec4(position, 1.0));
    gl_Position = projection * view * model * vec4(position, 1.0);

}