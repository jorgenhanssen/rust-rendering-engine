#version 430 core

in vec4 _color;
in vec3 _normal;

out vec4 Color;


vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

void main()
{
    Color = _color * max(0, dot(_normal, -lightDirection));
    Color.w = _color.w;
}