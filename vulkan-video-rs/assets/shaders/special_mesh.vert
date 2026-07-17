#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

layout(binding = 0) uniform Shared
{
    float Time;
    float Aspect;
} S;


bool show_norm = false;

vec3 light = vec3(-1, 0, -1);


#define TARGET_COL_COLOUR  0
#define TARGET_COL_NORMAL  1
#define TARGET_COL_DEPTH   2

#define TARGET 0


#define PI 3.141592653589793


void main() {

    float time   = S.Time;
    float period = radians(180) / 10;

    float period_s = 10;
    float theta = sin(time / period_s * PI / 2 - PI / 4) * PI / 4;
    float theta_else = 0; //cos(time / period_s * PI / 2 - PI / 4) * PI / 4;
    float FOV   = PI / 4;

    float ST = sin(time*period/2);
    float CT = cos(time*period/2);

    float STO = 1 - ST*ST;

    vec3  camera_pos = vec3(0, 0, 7);
    vec3  camera_dir = normalize(vec3(ST, 0, CT));

    vec3 rel_pos  = pos - camera_pos;

    // theta is the rotation around the y axis

    float SIN_Y = sin(theta);
    float COS_Y = cos(theta);

    mat3  view_x = mat3 ( COS_Y, 0, -SIN_Y,
                          0    , 1,  0,
                          SIN_Y, 0,  COS_Y);


    vec3 rot_pos  = view_x * rel_pos;


    float SIN_X = sin(theta_else);
    float COS_X = cos(theta_else);

    mat3  view_y = mat3 (1,      0,  0,
                         0,  COS_X,  SIN_X,
                         0, -SIN_X,  COS_X);

    view_y = transpose(view_y);
    rot_pos = view_y * rot_pos;
    rot_pos.y *= -1;


    vec4 world_pos = vec4(rot_pos, 1.0);

    mat4 view = create_view_matrix(camera_pos, camera_dir, vec3(0, 1, 0));
    world_pos = view * vec4(pos, 1.0);
    world_pos.z *= -1;


    mat4 proj = create_projection_matrix(FOV, S.Aspect);
    vec4 proj_pos = proj * world_pos;

    gl_Position = proj_pos;

    if (TARGET == TARGET_COL_COLOUR) {
        float light_cos = dot(normals,light);
        float alpha = ((light_cos * -1) + 1) / 2;
        float dark_factor = 0.8 * alpha;

        frag_color = 0.2 * vec3(col.x, col.y, col.z) * ( 1 - dark_factor);

    } else if (TARGET == TARGET_COL_DEPTH) {
        float dz_col = (rot_pos.z - camera_pos.z - 1) / 8.0;
        frag_color = vec3(dz_col, dz_col, dz_col);

    } else {
        frag_color = normals;
    }
}
