#version 460 core

in vec3 Colour;

out vec4 FragColor;
void main()
{
    FragColor = vec4(Colour, 0.8f);
}
