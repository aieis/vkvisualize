#version 460 core

layout (location = 0) in vec3 InPos;
layout (location = 1) in vec3 InColour;

out vec3 Colour;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(InPos, 1.0);
    Colour = InColour;
}

