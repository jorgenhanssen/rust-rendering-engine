#version 430 core

out vec4 color;

void main()
{
    float a = 1.0;
    int size = 20;

    int x = int(gl_FragCoord.x / size);
    int y = int(gl_FragCoord.y / size);

    if ((x + (y % 2)) % 2 == 0) {
        a = 0.0;
    }

    color = vec4(1.0f, 1.0f, 1.0f, a);
}