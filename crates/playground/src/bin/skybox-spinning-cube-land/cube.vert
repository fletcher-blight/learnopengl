#version 330 core

layout (location=1) in vec3 vertex;
layout (location=2) in vec3 position;
layout (location=3) in vec3 rotation_axis;
layout (location=4) in vec3 orbit_axis;

out vec2 tex_coords;

uniform mat4 projection;
uniform mat4 view;
uniform float rotation_angle;
uniform float orbit_angle;

mat4 rotate(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(
    oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
    oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
    oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
    0.0,                                0.0,                                0.0,                                1.0);
}

mat4 scale(float amount)
{
    return mat4(
    amount, 0.0, 0.0, 0.0,
    0.0, amount, 0.0, 0.0,
    0.0, 0.0, amount, 0.0,
    0.0, 0.0, 0.0, amount);
}

mat4 translate(vec3 offset)
{
    return mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    offset.x, offset.y, offset.z, 1.0);
}

void main() {
    switch (gl_VertexID % 6) {
        case 0: case 3: tex_coords = vec2(0.0, 0.0); break;
        case 1:         tex_coords = vec2(0.0, 1.0); break;
        case 2: case 4: tex_coords = vec2(1.0, 1.0); break;
        case 5:         tex_coords = vec2(1.0, 0.0); break;
    }
    gl_Position =
        projection * view
            * rotate(orbit_axis, orbit_angle)
                * translate(position)
                    * rotate(rotation_axis, rotation_angle)
                        * vec4(vertex, 1.0);
}