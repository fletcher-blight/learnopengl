#version 330 core

out vec4 colour;

in vec3 normal;
in vec3 frag_word_position;

uniform vec3 object_colour;
uniform vec3 light_colour;
uniform vec3 light_pos;

void main() {
    float ambient_strength = 0.1;

    vec3 light_dir = normalize(light_pos - frag_word_position);

    vec3 ambient_colour = ambient_strength * light_colour;
    vec3 diffuse_colour = max(dot(normal, light_dir), 0.0) * light_colour;

    vec3 result = (ambient_colour + diffuse_colour) * object_colour;
    colour = vec4(result, 1.0);
}