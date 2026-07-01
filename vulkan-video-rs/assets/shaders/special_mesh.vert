#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;

void main() {


    float time   = col.x;
    float period = radians(180/60);

    vec3  camera_pos = vec3(0, 0, 1.5);
    vec3  camera_dir = normalize(vec3(0, sin(time), 1));

    float cam_y_comp = length(camera_dir.xz); // ys compliment sqrt(x*x + z*z)
    float sin_x = - camera_dir.y;  // y / |v|
    float cos_x =   cam_y_comp;    // y_comp / |v|

    float sin_y = - camera_dir.x / cam_y_comp;
    float cos_y =   camera_dir.z / cam_y_comp;

    float y_comp   = length(pos.xz);
    float y_r      = y_comp * sin_x + pos.y * cos_x;
    float y_comp_r = y_comp * cos_x - pos.y * sin_x;
    float x_r      = pos.x; //pos.x / y_comp * y_comp_r;
    float z_r      = pos.z / y_comp * y_comp_r;
    vec3 rot_pos = vec3(x_r, y_r, z_r);

    float dz         = (rot_pos - camera_pos).z;

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = - focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  1 );

    vec3  proj_pos   = proj * rot_pos;

    gl_Position = vec4(proj_pos, 1.0);

    // frag_color = col;
    // frag_color = vec3(mod(col.x * 10, 60) / 60, 0.0, 1.0);
    frag_color = vec3(1.0, 0.0, 1.0);
}
