package main

import "core:c"
import glfw "vendor:glfw"


App :: struct  {
    should_close: bool,
    mouse_left_down: bool,
}

App_OnKey :: proc(app: ^App, key: i32, scancode: i32, action: i32, mods: c.int) {
    app.should_close = key == glfw.KEY_Q || key == glfw.KEY_ESCAPE;
}

App_OnMouse :: proc(app: ^App, button: i32, action: i32, mods: i32) {
    app.mouse_left_down = glfw.MOUSE_BUTTON_LEFT == button && action == glfw.PRESS 
}

App_Update :: proc(app: ^App) {
    
}

App_Draw :: proc(app: ^App) {
    
}
