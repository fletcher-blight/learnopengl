#version 330 core

in vec3 tex_coords;
out vec4 colour;

uniform samplerCube skybox;

void main() {
    colour = texture(skybox, tex_coords);
}