package main

import "core:fmt"
import "core:c"
import "core:math/linalg"

import glfw "vendor:glfw"
import gl "vendor:OpenGL"


App :: struct  {
    should_close: bool,
    mouse_left_down: bool,
    rotator: Rotator,
    shader: ShaderProgram,
    cube: Cube,
    camera: Camera,
    cached_mvp: matrix[4,4] f32
}

App_Create :: proc(aspect: f32) -> App {
    shader := ShaderProgram_Create(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG)
    cube := Cube_Create(0.5)
    camera := Camera_Create(aspect)
    cached_mvp := matrix[4,4] f32 {}


    return App {
        should_close = false,
        mouse_left_down = false,
        shader = shader,
        rotator = Rotator {pitch = 0, yaw = 0, roll = 0},
        cube = cube,
        camera = camera,
        cached_mvp = cached_mvp,
    }
}

App_OnKey :: proc(app: ^App, key: i32, scancode: i32, action: i32, mods: c.int) {
    app.should_close = key == glfw.KEY_ESCAPE;

    if key == glfw.KEY_A {
        app.rotator.yaw -= 10.0;
    }

    if key == glfw.KEY_D {
        app.rotator.yaw += 10.0;
    }


    if key == glfw.KEY_S {
        app.rotator.pitch -= 10.0;
    }

    if key == glfw.KEY_W {
        app.rotator.pitch += 10.0;
    }

    if key == glfw.KEY_Q {
        app.rotator.roll -= 10.0;
    }

    if key == glfw.KEY_E {
        app.rotator.roll += 10.0;
    }

    if key == glfw.KEY_T {
        app.rotator.pitch = 0
        app.rotator.roll = 0
        app.rotator.yaw = 0
    }
}

App_OnMouse :: proc(app: ^App, button: i32, action: i32, mods: i32) {
    app.mouse_left_down = glfw.MOUSE_BUTTON_LEFT == button && action == glfw.PRESS
}

App_OnWindowSize :: proc(app: ^App, width, height: i32) {
    Camera_Aspect(&app.camera, cast(f32) width / cast(f32) height)
}

App_Update :: proc(app: ^App) {
    quat := Quat_Normalized(Rotator_Quat(app.rotator))
    R := Quat_Matrix(quat)
    Cube_Rotate(&app.cube, R)
}

App_Draw :: proc(app: ^App) {
    gl.UseProgram(app.shader.id)
    if app.cached_mvp != app.camera.Mvp {
        app.cached_mvp = app.camera.Mvp
        gl.UniformMatrix4fv(ShaderProgram_UniformPosition(&app.shader, "mvp"), 1, false, &app.cached_mvp[0][0])
    }
    Cube_Draw(&app.cube)
}
