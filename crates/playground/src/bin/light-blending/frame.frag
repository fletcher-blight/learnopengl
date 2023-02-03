#version 330 core

out vec4 colour;
in vec2 v_tex_coords;

#define NUM_FRAMES 4
uniform sampler2D frame_texture[NUM_FRAMES];

void main() {
    vec4 res = vec4(0.0);
    for (int i = 0; i != NUM_FRAMES; ++i) {
        res += texture(frame_texture[i], v_tex_coords);
    }
    colour = res;
}