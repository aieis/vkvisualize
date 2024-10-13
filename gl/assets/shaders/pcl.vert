#version 460 core

layout (location = 0) in vec3 proj;
layout (location = 1) in float dist;

uniform mat4 mvp;
uniform vec3 col;

out vec3 vs_color;

void main()
{
    float norm = dist / 1000;
    gl_Position = mvp * vec4(proj.x * norm, proj.y * norm, norm, 1.0);
    if (abs(norm) < 0.001) {
        gl_PointSize = 0;
    } else {
        gl_PointSize =  0.5 / norm;
    }
    vs_color = col;
}
