#version 330 core

layout (location = 0) in vec3 position;

out vec3 v_tex_coords;

uniform mat4 view;
uniform mat4 projection;

void main() {
    v_tex_coords = position;
    vec4 result = projection * view * vec4(position, 1.0);
    gl_Position = result.xyww;
}