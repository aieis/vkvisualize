package main

import "core:fmt"
import "base:runtime"
import "core:c"

import glfw "vendor:glfw"
import gl "vendor:OpenGL"

main :: proc() {
    glfw.Init();
    defer glfw.Terminate()

    window := glfw.CreateWindow(1024, 1024, "Hello, Odin", nil, nil);
    defer glfw.DestroyWindow(window)

    glfw.MakeContextCurrent(window)
    glfw.WindowHint(glfw.CONTEXT_VERSION_MAJOR, 4)
    glfw.WindowHint(glfw.CONTEXT_VERSION_MINOR, 6)
    glfw.WindowHint(glfw.OPENGL_PROFILE, glfw.OPENGL_CORE_PROFILE);
    gl.load_up_to(4, 6, glfw.gl_set_proc_address)

    app := App {should_close=false}

    glfw.SetWindowUserPointer(window, cast(rawptr) &app)
    glfw.SetKeyCallback(window, OnKeyPress)
    glfw.SetMouseButtonCallback(window, OnMouseButton)
    
    gl.ClearColor(0.5, 0.5, 0.5, 0.5);

    shader := ShaderProgram_Create(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG)
    cube := Cube_Create(0.5)

    for {

        if glfw.WindowShouldClose(window) || app.should_close {
            break;
        }

        glfw.PollEvents();

        gl.Clear(gl.COLOR_BUFFER_BIT)
        gl.UseProgram(shader.id)
        Cube_Draw(&cube)
        glfw.SwapBuffers(window);
    }

}

OnKeyPress :: proc "c" (window: glfw.WindowHandle, key: i32, scancode: i32, action: i32, mods: i32) {
    context = runtime.default_context()
    app := cast (^App) glfw.GetWindowUserPointer(window)
    App_OnKey(app, key, scancode, action, mods)
}

OnMouseButton :: proc "c" (window: glfw.WindowHandle, button: i32, action: i32, mods: i32) {
    context = runtime.default_context()
    app := cast (^App) glfw.GetWindowUserPointer(window)
    App_OnMouse(app, button, action, mods)    
}
