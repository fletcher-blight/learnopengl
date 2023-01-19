#version 330 core

out vec4 colour;

in vec3 normal;
in vec3 frag_word_position;

uniform vec3 object_colour;
uniform vec3 light_colour;
uniform vec3 light_pos;
uniform vec3 view_pos;

void main() {
    float ambient_strength = 0.1;
    float specular_strength = 0.5;
    float specular_shininess = 32;

    vec3 normal_dir = normalize(normal);
    vec3 light_dir = normalize(light_pos - frag_word_position);
    vec3 view_dir = normalize(view_pos - frag_word_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    vec3 ambient_colour = ambient_strength * light_colour;
    vec3 diffuse_colour = max(dot(normal_dir, light_dir), 0.0) * light_colour;
    vec3 specular_colour = specular_strength * pow(max(dot(view_dir, reflect_dir), 0.0), specular_shininess) * light_colour;

    vec3 result = (ambient_colour + diffuse_colour + specular_colour) * object_colour;
    colour = vec4(result, 1.0);
}