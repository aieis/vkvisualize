package main

SHADER_TEXTURE_VERT :: #load("../assets/shaders/tex.vert")
SHADER_TEXTURE_FRAG :: #load("../assets/shaders/tex.frag")


Texture :: struct {
    id: u32,
    vbo: u32,
    vao: u32,
    width: u32,
    height: u32,
    format: u32
}


Texture_Create :: proc () {
    gl
}
