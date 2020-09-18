#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;

out vec4 out_color;

void main()
{
    gl_Position = vec4(position, 1.0f);
    out_color = color;
}
