#version 330 core

out vec2 tex_coords;

vec4 get_position(in mat4 transform, in vec3 offset) {
    return transform * (gl_in[0].gl_Position + vec4(offset, 0.0));
}

void emit_offsets(
    in vec3 bottom_left,
    in vec3 top_left,
    in vec3 top_right,
    in vec3 bottom_right,
    in mat4 transform)
{
    gl_Position = get_position(transform, bottom_left);
    tex_coords = vec2(0.0, 0.0);
    EmitVertex();
    gl_Position = get_position(transform, top_left);
    tex_coords = vec2(0.0, 1.0);
    EmitVertex();
    gl_Position = get_position(transform, top_right);
    tex_coords = vec2(1.0, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = get_position(transform, bottom_left);
    tex_coords = vec2(0.0, 0.0);
    EmitVertex();
    gl_Position = get_position(transform, top_right);
    tex_coords = vec2(1.0, 1.0);
    EmitVertex();
    gl_Position = get_position(transform, bottom_right);
    tex_coords = vec2(1.0, 0.0);
    EmitVertex();
    EndPrimitive();
}