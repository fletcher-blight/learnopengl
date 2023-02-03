
#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texture_coordinates;

out vec3 frag_normal;
out vec2 frag_texture_coordinates;
out vec3 frag_word_position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    frag_normal = mat3(transpose(inverse(model))) * normal;
    frag_texture_coordinates = texture_coordinates;
    frag_word_position = vec3(model * vec4(position, 1.0));
    gl_Position = projection * view * model * vec4(position, 1.0);
}