#version 330 core

in vec2 v_tex_coords;
out vec4 colour;

uniform sampler2D tex1;
uniform sampler2D tex2;

void main() {
    colour = mix(texture(tex1, v_tex_coords), texture(tex2, v_tex_coords), 0.4);
}