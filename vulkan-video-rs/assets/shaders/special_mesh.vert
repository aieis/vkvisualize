#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

bool show_norm = false;

vec3 light = vec3(0, 1, -1);


#define TARGET_COL_COLOUR  0
#define TARGET_COL_NORMAL  1
#define TARGET_COL_DEPTH   2

#define TARGET 0


void main() {

    float time   = col.x;
    float period = radians(180) / 10;

    float ST = sin(time*period/2);
    float CT = cos(time*period);

    float STO = 1 - ST*ST;

    vec3  camera_pos = vec3(0, 0, 7);
    vec3  camera_dir = normalize(vec3(0, 0, -1));
    vec3  forward_dir = vec3(0, 0, 1);

    vec3 rel_pos  = pos - camera_pos;
    

    float SIN_X = camera_dir.x;
    float COS_X = camera_dir.z;

    mat3  view_x = mat3 ( COS_X, 0,  SIN_X,
                          0    , 1,  0,
                         -SIN_X, 0,  COS_X);

    view_x = transpose(view_x);
    vec3 rot_pos  = view_x * rel_pos;
    rot_pos.x *= -1;

    float SIN_Y = camera_dir.y;
    float COS_Y = camera_dir.z;

    mat3  view_y = mat3 (1    , 0,  0,
                         0, COS_Y, -SIN_Y,
                         0, SIN_Y,  COS_Y
                         );



    float dz        = abs(rel_pos.z);

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  0 );

    vec3  proj_pos   = proj * rot_pos;

    gl_Position = vec4(proj_pos, 1.0);


    if (TARGET == TARGET_COL_COLOUR) {
        float light_cos = dot(normals,light);
        float alpha = ((light_cos * -1) + 1) / 2;
        float dark_factor = 0.5 * alpha;

        frag_color = vec3(col.x, col.y, col.z) * ( 1 - dark_factor);

    } else if (TARGET == TARGET_COL_DEPTH) {
        float dz_col = (dz - camera_pos.z - 1) / 8.0;
        frag_color = vec3(dz_col, dz_col, dz_col);

    } else {
        frag_color = normals;
    }
}
