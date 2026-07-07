#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;


void main() {


    float time   = col.x;
    float period = radians(180) / 10;

    float ST = sin(time*period);
    float CT = cos(time*period);

    float STO = 1 - ST*ST;

    vec3  camera_pos = vec3(0, 3, -5);
    vec3  camera_dir = normalize(vec3(0, 1, 1));
    vec3  forward_dir = vec3(0, 0, 1);


    float SIN_X = camera_dir.x;
    float COS_X = camera_dir.z;

    mat3  view_x = mat3 ( COS_X, 0,  SIN_X,
                          0    , 1,  0,
                         -SIN_X, 0,  COS_X);

    float SIN_Y = camera_dir.y;
    float COS_Y = camera_dir.z;

    mat3  view_y = mat3 (1    , 0,  0,
                         0, COS_Y, -SIN_Y,
                         0, SIN_Y,  COS_Y
                         );


    vec3 rel_pos  = pos - camera_pos;
    vec3 rot_pos  = view_x * rel_pos;

    float dz        = abs(rot_pos.z);

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = - focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  0 );

    vec3  proj_pos   = proj * rot_pos;

    gl_Position = vec4(proj_pos, 1.0);

    // frag_color = col;
    // frag_color = vec3(mod(col.x * 10, 60) / 60, 0.0, 1.0);
    frag_color = vec3(0.0, col.y, col.z);
}
