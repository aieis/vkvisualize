#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec2 pos;
layout(location = 1) in vec3 col;

void main() {

    gl_Position = vec4(pos, 0.0, 1.0);
    frag_color = col;
}
