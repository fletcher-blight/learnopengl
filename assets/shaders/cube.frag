#version 330 core

in vec2 v_tex_coords;
out vec4 colour;

uniform sampler2D tex1;

void main() {
    colour = texture(tex1, v_tex_coords);
}