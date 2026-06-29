#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;

void main() {

    vec3  camera_pos = vec3(0, 0, 1);

    float dz         = (pos - camera_pos).z;

    if (dz == 0) {
        dz = 0.1;
    }

    float fx         = 0.4;

    vec3  proj_pos   = vec3(-pos.x * fx / dz, -pos.y * fx / dz, pos.z);

    gl_Position = vec4(proj_pos, 1.0);

    frag_color = col;
}
