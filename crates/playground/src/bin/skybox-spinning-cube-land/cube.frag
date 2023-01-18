#version 330 core

out vec4 colour;
in vec2 tex_coords;

uniform sampler2D tex;

void main() {
    colour = texture(tex, tex_coords);
}