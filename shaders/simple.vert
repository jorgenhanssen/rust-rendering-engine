#version 430 core

in vec3 position;

void main()
{
    float x = position.x / 4;
    float y = position.y / 4;
    float z = position.z / 4;
    gl_Position = vec4(x, y, z, 1.0f);
}