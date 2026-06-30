#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive: enable
// #include "./utils/common.glsl"

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;

void main() {

    vec3  camera_pos = vec3(0, 0, 1);
    vec3  camera_dir = normalize(vec3(0, 0, -1));

    // float cam_y_comp = length(camera_dir.xz); // ys complement sqrt(x*x + z * z)
    // float sin_x = camera_dir.y;  // y / |v|
    // float cos_x = cam_y_comp;    // y_comp / |v|

    // float sin_y = camera_dir.x / cam_y_comp;
    // float cos_y = camera_dir.z / cam_y_comp;

    float dz         = (pos - camera_pos).z;

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = - focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  1 );

    vec3  proj_pos   = proj * pos;

    gl_Position = vec4(proj_pos, 1.0);

    frag_color = col;
}
