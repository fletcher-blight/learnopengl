#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

out vec4 colour;

in vec3 normal;
in vec3 frag_word_position;

uniform vec3 view_pos;
uniform Material material;
uniform Light light;

void main() {
    vec3 normal_dir = normalize(normal);
    vec3 light_dir = normalize(light.position - frag_word_position);
    vec3 view_dir = normalize(view_pos - frag_word_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    vec3 ambient_colour = light.ambient * material.ambient;
    vec3 diffuse_colour = light.diffuse * material.diffuse * max(dot(normal_dir, light_dir), 0.0);
    vec3 specular_colour = light.specular * material.specular * pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    vec3 result = ambient_colour + diffuse_colour + specular_colour;
    colour = vec4(result, 1.0);
}