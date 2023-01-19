#version 330 core

out vec4 colour;

uniform vec3 object_colour;
uniform vec3 light_colour;

void main() {
    vec3 result = object_colour * light_colour;
    colour = vec4(result, 1.0);
}