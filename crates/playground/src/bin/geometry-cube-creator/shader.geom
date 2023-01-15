#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 36) out;

out vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void emit_offsets(
    in vec3 bottom_left,
    in vec3 top_left,
    in vec3 top_right,
    in vec3 bottom_right,
    in mat4 transform)
{
    gl_Position = transform * (gl_in[0].gl_Position + vec4(bottom_left, 0.0));
    tex_coords = vec2(0.0, 0.0);
    EmitVertex();
    gl_Position = transform * (gl_in[0].gl_Position + vec4(top_left, 0.0));
    tex_coords = vec2(0.0, 1.0);
    EmitVertex();
    gl_Position = transform * (gl_in[0].gl_Position + vec4(top_right, 0.0));
    tex_coords = vec2(1.0, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = transform * (gl_in[0].gl_Position + vec4(bottom_left, 0.0));
    tex_coords = vec2(0.0, 0.0);
    EmitVertex();
    gl_Position = transform * (gl_in[0].gl_Position + vec4(top_right, 0.0));
    tex_coords = vec2(1.0, 1.0);
    EmitVertex();
    gl_Position = transform * (gl_in[0].gl_Position + vec4(bottom_right, 0.0));
    tex_coords = vec2(1.0, 0.0);
    EmitVertex();
    EndPrimitive();
}

void main() {
    mat4 transform = projection * view * model;

    emit_offsets(
        vec3(-0.5, -0.5, 0.5),
        vec3(-0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, -0.5, 0.5),
        transform);

    emit_offsets(
        vec3(-0.5, -0.5, -0.5),
        vec3(-0.5, 0.5, -0.5),
        vec3(-0.5, 0.5, 0.5),
        vec3(-0.5, -0.5, 0.5),
        transform);

    emit_offsets(
        vec3(-0.5, 0.5, 0.5),
        vec3(-0.5, 0.5, -0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(0.5, 0.5, 0.5),
        transform);

    emit_offsets(
        vec3(0.5, -0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(0.5, -0.5, -0.5),
        transform);

    emit_offsets(
        vec3(0.5, -0.5, 0.5),
        vec3(0.5, -0.5, -0.5),
        vec3(-0.5, -0.5, -0.5),
        vec3(-0.5, -0.5, 0.5),
        transform);

    emit_offsets(
        vec3(0.5, -0.5, -0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(-0.5, 0.5, -0.5),
        vec3(-0.5, -0.5, -0.5),
        transform);
}