#version 460 core

layout (location = 0) in vec3 InPos;
layout (location = 1) in vec2 InTexCoord;

out vec2 TexCoord;

void main()
{
    gl_Position = vec4(InPos, 1.0);
    TexCoord = InTexCoord;
}

