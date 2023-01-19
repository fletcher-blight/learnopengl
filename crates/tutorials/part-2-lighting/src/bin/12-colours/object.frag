#version 330 core

out vec4 colour;
in vec3 v_normal;

uniform vec3 object_colour;
uniform vec3 light_colour;
uniform vec3 light_pos;

void main() {
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_colour;

    vec3 result = ambient * object_colour;
    colour = vec4(result, 1.0);
}