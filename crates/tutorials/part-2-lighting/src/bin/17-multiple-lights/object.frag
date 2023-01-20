#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float cutoff;
    float outer_cutoff;

    float attenuation_linear;
    float attenuation_quadratic;
};

struct DirLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
uniform DirLight dir_light;

vec3 calculate_dir_light(DirLight light, vec3 normal, vec3 view_dir) {
    vec3 light_dir = normalize(-light.direction);
    vec3 view_dir = normalize(view_pos - frag_word_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);

    vec3 diffuse_texture = vec3(texture(material.diffuse, tex_coord));
    vec3 specular_texture = vec3(texture(material.specular, tex_coord));

    vec3 ambient_colour = light.ambient * diffuse_texture;
    vec3 diffuse_colour = light.diffuse * diffuse_texture * max(dot(normal_dir, light_dir), 0.0);
    vec3 specular_colour = light.specular * specular_texture * pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    return ambient_colour + diffuse_colour + specular_colour;
}

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float attenuation_linear;
    float attenuation_quadratic;
};
#define NUM_POINT_LIGHTS 4
uniform PointLight point_lights[NUM_POINT_LIGHTS];

vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_word_position) {

}

out vec4 colour;

in vec3 normal;
in vec3 frag_word_position;
in vec2 tex_coord;

uniform vec3 view_pos;
uniform Material material;
uniform Light light;

void main() {
    vec3 normal_dir = normalize(normal);
    vec3 light_dir = normalize(light.position - frag_word_position);
    vec3 spot_dir = normalize(-light.direction);
    vec3 view_dir = normalize(view_pos - frag_word_position);
    vec3 reflect_dir = reflect(-light_dir, normal_dir);
    float theta = dot(light_dir, spot_dir);
    float epsilon = light.cutoff - light.outer_cutoff;
    float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);
    float distance = length(light.position - frag_word_position);
    float attenuation = 1.0 / (1.0 + (light.attenuation_linear * distance) + (light.attenuation_quadratic * distance * distance));

    vec3 diffuse_texture = vec3(texture(material.diffuse, tex_coord));
    vec3 specular_texture = vec3(texture(material.specular, tex_coord));

    vec3 ambient_colour = light.ambient * diffuse_texture;
    vec3 diffuse_colour = intensity * light.diffuse * diffuse_texture * max(dot(normal_dir, light_dir), 0.0);
    vec3 specular_colour = intensity * light.specular * specular_texture * pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);

    colour = vec4(attenuation * (ambient_colour + diffuse_colour + specular_colour), 1.0);
}