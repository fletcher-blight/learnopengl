#version 330 core

out vec4 colour;

uniform vec3 flavour;

void main() {
    colour = vec4(flavour, 1.0);
}