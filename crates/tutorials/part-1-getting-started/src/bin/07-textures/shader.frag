#version 330 core

out vec4 colour;
in vec2 v_tex_coords;

uniform sampler2D tex;

void main() {
    colour = texture(tex, v_tex_coords);
}