#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float attenuation_linear;
    float attenuation_quadratic;
};

out vec4 colour;

in vec3 frag_normal;
in vec2 frag_texture_coordinates;
in vec3 frag_word_position;

uniform vec3 view_pos;
uniform Material material;
uniform Light light;

void main() {
    vec3 normal_dir = normalize(frag_normal);
    vec3 light_dir = normalize(light.position - frag_word_position);
    vec3 view_dir = normalize(view_pos - frag_word_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);
    float distance = length(light.position - frag_word_position);
    float attenuation = 1.0 / (1.0 + (light.attenuation_linear * distance) + (light.attenuation_quadratic * distance * distance));

    vec3 diffuse_texture = vec3(texture(material.diffuse, frag_texture_coordinates));
    vec3 specular_texture = vec3(texture(material.specular, frag_texture_coordinates));

    vec3 ambient_colour = light.ambient * diffuse_texture;
    vec3 diffuse_colour = light.diffuse * diffuse_texture * max(dot(normal_dir, light_dir), 0.0);
    vec3 specular_colour = light.specular * specular_texture * pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    colour = vec4(attenuation * (ambient_colour + diffuse_colour + specular_colour), 1.0);
}