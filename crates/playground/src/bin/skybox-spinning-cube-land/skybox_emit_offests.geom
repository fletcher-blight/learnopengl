#version 330 core

out vec3 tex_coords;

vec4 get_position(in mat4 transform, in vec3 offset) {
    vec4 position = transform * (gl_in[0].gl_Position + vec4(offset, 0.0));
    tex_coords = vec3(position.xyz);
    return position.xyww;
}

void emit_offsets(
    in vec3 bottom_left,
    in vec3 top_left,
    in vec3 top_right,
    in vec3 bottom_right,
    in mat4 transform)
{
    gl_Position = get_position(transform, bottom_left);
    EmitVertex();
    gl_Position = get_position(transform, top_left);
    EmitVertex();
    gl_Position = get_position(transform, top_right);
    EmitVertex();
    EndPrimitive();

    gl_Position = get_position(transform, bottom_left);
    EmitVertex();
    gl_Position = get_position(transform, top_right);
    EmitVertex();
    gl_Position = get_position(transform, bottom_right);
    EmitVertex();
    EndPrimitive();
}