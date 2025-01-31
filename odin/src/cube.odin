package main

SHADER_SIMPLE_VERT :: #load("../assets/shaders/simple.vert")
SHADER_SIMPLE_FRAG :: #load("../assets/shaders/simple.frag")

import gl "vendor:OpenGL"

Cube :: struct {
    pos_vbo: u32,
    col_vbo: u32,
    ebo: u32,
    vao: u32,
    vertices: [24]f32,
    indices: [36]u32,
    colours: [24]f32
}

Cube_Create :: proc(dim: f32) -> Cube {

    vertices := [24]f32 {
        -dim, -dim, -dim,
        +dim, -dim, -dim,
        +dim, +dim, -dim,
        -dim, +dim, -dim,
        -dim, -dim, +dim,
        +dim, -dim, +dim,
        +dim, +dim, +dim,
        -dim, +dim, +dim
    }

    colours := [24]f32 {
        0.5, 0.5, 0.5,
        1.0, 0.5, 0.5,
        1.0, 1.0, 0.5,
        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        1.0, 0.5, 1.0,
        1.0, 1.0, 1.0,
        0.5, 1.0, 1.0
    }

    indices := [36]u32 {
        0, 1, 2, 0, 3, 2,
        0, 4, 5, 0, 6, 5,
        0, 4, 7, 0, 3, 7,
        6, 4, 5, 6, 4, 7,
        6, 3, 7, 6, 3, 2,
        6, 1, 5, 6, 1, 2
    }

    pos_vbo : u32
    gl.CreateBuffers(1, &pos_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, pos_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, 24 * size_of(f32), &vertices, gl.STATIC_DRAW)

    col_vbo : u32
    gl.CreateBuffers(1, &col_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, col_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, 24 * size_of(f32), &colours, gl.STATIC_DRAW)

    ebo : u32
    gl.CreateBuffers(1, &ebo)
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo)
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, 36 * size_of(u32), &indices, gl.STATIC_DRAW)

    vao: u32
    gl.CreateVertexArrays(1, &vao)
    gl.BindVertexArray(vao)

    gl.BindBuffer(gl.ARRAY_BUFFER, pos_vbo)
    gl.VertexAttribPointer(0, 3, gl.FLOAT, gl.FALSE, 3 * size_of(f32), 0)
    gl.EnableVertexAttribArray(0)

    gl.BindBuffer(gl.ARRAY_BUFFER, col_vbo)
    gl.VertexAttribPointer(1, 3, gl.FLOAT, gl.FALSE, 3 * size_of(f32), 0)
    gl.EnableVertexAttribArray(1)

    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo)
    gl.BindVertexArray(0)

    return Cube {
        col_vbo,
        pos_vbo,
        ebo,
        vao,
        vertices,
        indices,
        colours
    }
}

Cube_Draw :: proc(cube: ^Cube) {
    gl.BindVertexArray(cube.vao)
    gl.DrawElements(gl.TRIANGLES, size_of(cube.indices) / size_of(u32), gl.UNSIGNED_INT, nil)
    gl.BindVertexArray(0)
}
