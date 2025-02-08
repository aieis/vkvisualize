package main

import "core:math"
import la "core:math/linalg"

Camera :: struct {
    Projection: matrix[4, 4] f32,
    View: matrix[4, 4] f32,
    Model: matrix[4, 4] f32,
    Mvp: matrix[4, 4] f32
}

Camera_Create :: proc(aspect: f32) -> Camera {
    Projection := Perspective(math.to_radians_f32(45), aspect, 0.1, 100)
    View := Look_At({0, 0, 1}, {0, 0, 0}, {0, 1, 0})
    Model := la.identity(matrix[4,4] f32)
    Mvp := la.matrix_mul(la.matrix_mul(Projection, View), Model)

    return Camera {
        Projection = Projection,
        View = View,
        Model = Model,
        Mvp = Mvp
    }
}

Camera_Aspect :: proc(camera: ^Camera, aspect: f32)  {
    camera.Projection = Perspective(math.to_radians_f32(45), aspect, 0.1, 100)    
}

Camera_Mvp :: proc(cam: ^Camera) -> matrix[4,4] f32 {
    cam.Mvp = la.matrix_mul(la.matrix_mul(cam.Projection, cam.View), cam.Model)
    return cam.Mvp
}

Perspective :: proc(fovy: f32, aspect: f32, nearz: f32, farz: f32) -> matrix[4,4] f32 {
    dest := matrix [4, 4] f32 {};

    f := 1.0 / math.tan(fovy * 0.5)
    fn := 1.0 / (nearz - farz)
    
    dest[0][0] = f / aspect
    dest[1][1] = f
    dest[2][2] =-farz * fn
    dest[2][3] = 1.0
    dest[3][2] = nearz * farz * fn
    
    return dest
}

Look_At :: proc(eye: [3] f32, center: [3]f32, up: [3]f32) -> matrix[4,4]f32 {
    dest := matrix [4, 4] f32 {};
    
    f := center - eye //glm_vec3_sub(center, eye, f);
    f = la.normalize(f) //glm_vec3_normalize(f);
    s := la.cross(up, f) //glm_vec3_crossn(up, f, s);
    u := la.cross(f, s) //glm_vec3_cross(f, s, u);

    dest[0][0] = s[0];
    dest[0][1] = u[0];
    dest[0][2] = f[0];
    dest[1][0] = s[1];
    dest[1][1] = u[1];
    dest[1][2] = f[1];
    dest[2][0] = s[2];
    dest[2][1] = u[2];
    dest[2][2] = f[2];
    dest[3][0] =-la.dot(s, eye);
    dest[3][1] =-la.dot(u, eye);
    dest[3][2] =-la.dot(f, eye);
    dest[0][3] = 0; dest[1][3] = 0;  dest[2][3] = 0;
    dest[3][3] = 1.0

    return dest
}

