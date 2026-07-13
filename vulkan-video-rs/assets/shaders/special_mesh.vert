#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

bool show_norm = false;

vec3 light = vec3(0, 1, -1);

mat3 look_at(vec3 origin, vec3 target, float roll) {
  vec3 rr = vec3(sin(roll), cos(roll), 0.0);
  vec3 ww = normalize(target - origin);
  vec3 uu = normalize(cross(ww, rr));
  vec3 vv = normalize(cross(uu, ww));

  return mat3(uu, vv, ww);
}

mat4 look_at_2(vec3 eye, vec3 center, vec3 up) {
    mat4 dest = mat4(0.0);

    vec3 f = normalize(center - eye);
    vec3 s = cross(up, f);
    vec3 u = cross(f, s);

    dest[0][0] = s.x;
    dest[0][1] = u.x;
    dest[0][2] = f.x;

    dest[1][0] = s.y;
    dest[1][1] = u.y;
    dest[1][2] = f.y;

    dest[2][0] = s.z;
    dest[2][1] = u.z;
    dest[2][2] = f.z;

    dest[3][0] = -dot(s, eye);
    dest[3][1] = -dot(u, eye);
    dest[3][2] = -dot(f, eye);

    dest[0][3] = 0.0;
    dest[1][3] = 0.0;
    dest[2][3] = 0.0;
    dest[3][3] = 1.0;

    return dest;
}

mat4 projection_2(float fovy, float aspect) {
    mat4 dest = mat4(0.0);

    float farz = 100.0;
    float nearz = 0.1;


    float f = 1.0 / tan(fovy * 0.5);
    float fn = 1.0 / (nearz - farz);

    dest[0][0] = f / aspect;
    dest[1][1] = f;
    dest[2][2] = -farz * fn;
    dest[2][3] = 1.0;
    dest[3][2] = nearz * farz * fn;

    return dest;
}

mat4 projection(float fov_y, float aspect) {
    float f = cos(fov_y / 2) / sin(fov_y / 2);

    float zf = 150;
    float zn = 0.1;

    float t1 = (zf + zn) / (zn - zf);
    float t2 = (2*zf*zn) / (zn - zf);

    mat4 proj = mat4(f/aspect, 0, 0,  0,
                     0       , f, 0,  0,
                     0       , 0, t1, t2,
                     0       , 0, -1, 0);

    return proj;
}


void main() {


    float time   = col.x;
    float period = radians(180) / 10;

    float ST = sin(time*period/2);
    float CT = cos(time*period);

    float STO = 1 - ST*ST;

    vec3  camera_pos = vec3(0.2, -2, 3);
    vec3  camera_dir = normalize(vec3(0, 0.5, -1));
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

    vec3 real_pos = vec3(pos.x, -pos.y, pos.z);

    vec3 rel_pos  = real_pos; //pos - camera_pos;

    mat3 view = look_at(camera_pos, camera_pos + camera_dir, 0);

    vec3 rot_pos  = view * rel_pos;

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


    mat4 proj_p = projection_2(radians(45), 1);
    mat4 view_2 = look_at_2(camera_pos, camera_pos+camera_dir, vec3(0, 1, 0));

    vec4 pre_ready = proj_p * view_2 * vec4(real_pos, 1.0);
    gl_Position = pre_ready;
    // gl_Position = vec4(proj_pos, 1.0);

    // frag_color = col;
    // frag_color = vec3(mod(col.x * 10, 60) / 60, 0.0, 1.0);

    if (!show_norm) {

        float light_cos = dot(normals,light);
        float alpha = ((light_cos * -1) + 1) / 2;
        float dark_factor = 0.8 * alpha;
        frag_color = vec3(col.x, col.y, col.z) * ( 1 - dark_factor);
    } else {
        frag_color = normals;
    }
}
