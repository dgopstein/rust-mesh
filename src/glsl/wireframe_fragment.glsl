#version 330

in Data
{
    noperspective in vec3 dist;
} gdata;

out vec4 outputColor;

uniform sampler2D tex;

const vec4 wireframeColor = vec4(1.0f, 0.5f, 0.5f, 1.0f);
const vec4 fillColor = vec4(1.0f, 1.0f, 1.0f, 0.0f);

void main()
{
    float d = min(gdata.dist.x, min(gdata.dist.y, gdata.dist.z));
    float I = exp2(-2*d*d);
    outputColor = mix(fillColor, wireframeColor, I);
}
