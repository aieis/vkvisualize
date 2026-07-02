#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;

void main() {


    float time   = col.x;
    float period = radians(180) / 10;

    vec3  camera_pos = vec3(0, 0, -5);
    vec3  camera_dir = normalize(vec3(sin(time*period), 0, 1));

    float cam_y_comp = length(camera_dir.xz); // ys compliment sqrt(x*x + z*z)
    float sin_x = - camera_dir.y;  // y / |v|
    float cos_x =   cam_y_comp;    // y_comp / |v|

    float sin_y = - camera_dir.x / cam_y_comp;
    float cos_y =   camera_dir.z / cam_y_comp;

    vec3  rel_pos  = pos - camera_pos;
    float y_comp   = length(rel_pos.xz);
    float y_r      = y_comp * sin_x + rel_pos.y * cos_x;
    float y_comp_r = y_comp * cos_x - rel_pos.y * sin_x;
    float x_r      = rel_pos.x / y_comp * y_comp_r;
    float z_r      = rel_pos.z / y_comp * y_comp_r;
    vec3  rot_pos  = vec3(x_r, y_r, z_r);

    float x_final  = x_r * cos_y - z_r * sin_y;
    float z_final  = x_r * sin_y + z_r * cos_y;
    float y_final  = y_r;

    rot_pos = vec3(x_final, y_final, z_final);

    float dz        = rot_pos.z;

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = - focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  1/15 );

    vec3  proj_pos   = proj * rot_pos;

    gl_Position = vec4(proj_pos, 1.0);

    // frag_color = col;
    // frag_color = vec3(mod(col.x * 10, 60) / 60, 0.0, 1.0);
    frag_color = vec3(1.0, 0.0, 1.0);
}
