#version 330 core

out vec4 colour;
in vec2 v_tex_coords;

uniform sampler2D tex1;
uniform sampler2D tex2;

void main() {
    colour = texture(tex1, v_tex_coords) + texture(tex2, v_tex_coords);
}