#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec3 colour;

out vec3 v_colour;

void main() {
    v_colour = colour;
    gl_Position = vec4(position, 0.0, 1.0);
}