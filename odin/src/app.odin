package main

import "core:c"

import glfw "vendor:glfw"
import gl "vendor:OpenGL"

import "./mesh"


App :: struct  {
    should_close: bool,
    mouse_left_down: bool,

    rotator: Rotator,
    shader: ShaderProgram,
    cube: Cube,
    thing: Thing
}

App_Create :: proc() -> App {
    shader := ShaderProgram_Create(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG)
    cube := Cube_Create(0.5)

    cone := mesh.Cone_Create(0.5, 0.5)
    thing := Thing_Create(cone.vertices, cone.triangles, {0.1, 0.9, 0.1})

    return App {
        should_close = false,
        mouse_left_down = false,
        shader = shader,
        cube = cube,
        thing = thing
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

App_Update :: proc(app: ^App) {
    quat := Quat_Normalized(Rotator_Quat(app.rotator))
    R := Quat_Matrix(quat)
    Cube_Rotate(&app.cube, R)
}

App_Draw :: proc(app: ^App) {
    gl.UseProgram(app.shader.id)
    Cube_Draw(&app.cube)
    // Thing_Draw(&app.thing)
}
