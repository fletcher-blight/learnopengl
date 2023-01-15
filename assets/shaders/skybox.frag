#version 330 core
in vec3 v_tex_coords;

out vec4 colour;

uniform samplerCube skybox;

void main() {
    colour = texture(skybox, v_tex_coords);
}