package main;

import "core:fmt"
import "core:strings"

import gl "vendor:OpenGL"

ShaderProgram :: struct {
    id: u32
}

ShaderProgram_Create :: proc(vert_src: []u8, frag_src: []u8) -> ShaderProgram {
    prog := ShaderProgram { id = 0 }

    svert, vert := Shader_Create(vert_src, gl.VERTEX_SHADER)
    if !svert {
        return prog
    }

    defer gl.DeleteShader(vert)

    sfrag, frag := Shader_Create(vert_src, gl.VERTEX_SHADER)
    if !sfrag {
        return prog;
    }

    defer gl.DeleteShader(frag)

    id := gl.CreateProgram()

    gl.AttachShader(id, vert)
    gl.AttachShader(id, frag)
    gl.LinkProgram(id)

    status : i32 = 0
    gl.GetProgramiv(id, gl.LINK_STATUS, &status)
    msg : [512]u8
    if status == 0 {
        read : i32 = 0
        gl.GetProgramInfoLog(id, 511, &read, &msg[0])
        msg_string := strings.string_from_ptr(&msg[0], cast(int) read)
        fmt.eprintfln("Failed to link program: %s", msg_string)
    }

    return ShaderProgram { id = id }
}

ShaderProgram_Delete :: proc(prog: ShaderProgram) {
    gl.DeleteProgram(prog.id)
}

Shader_Create :: proc(src: []u8, type: u32) -> (bool, u32) {
    shader := gl.CreateShader(type)
    length := cast(i32) len(src)
    gl.ShaderSource(shader, 1, cast(^cstring)&src[0], &length)
    gl.CompileShader(shader)

    status : i32 = 0
    gl.GetShaderiv(shader, gl.COMPILE_STATUS, &status)
    msg : [512]u8
    if status == 0 {
        read : i32 = 0
        gl.GetShaderInfoLog(shader, 511, &read, &msg[0])
        msg_string := strings.string_from_ptr(&msg[0], cast(int) read)
        fmt.eprintfln("Failed to compile shader: %s", msg_string)
        gl.DeleteShader(shader)
        return false, 0
    }

    return true, shader
}
