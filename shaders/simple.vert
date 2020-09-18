#version 430 core

in layout (location = 0) vec3 position;
in layout (location = 1) vec4 color;

out vec4 out_color;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(position, 1.0f);
    out_color = color;
}
