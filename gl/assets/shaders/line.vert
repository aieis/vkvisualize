#version 460 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 col;

uniform mat4 mvp;

out vec3 vs_color;

void main()
{
    gl_Position = mvp * vec4(pos, 1.0);
    vs_color = col;
}
