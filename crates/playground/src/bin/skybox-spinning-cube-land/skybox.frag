#version 330 core

out vec4 colour;
in vec3 tex_coords;

uniform samplerCube skybox;

void main() {
    colour = texture(skybox, tex_coords);
}