#version 330

layout(location = 0) in vec4 position;

uniform mat4 matrix;

out Data
{
    vec4 position;
} vdata;

void main()
{
    vdata.position = matrix * position;
}
