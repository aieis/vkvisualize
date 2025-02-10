package main

import gl "vendor:OpenGL"
import "core:math/linalg"

import "./mesh"

Thing :: struct {
    pos_vbo: u32,
    col_vbo: u32,
    ebo: u32,
    vao: u32,
    vertices: [dynamic] f32,
    triangles: [dynamic] u32,
    colours: [dynamic] f32,
}

Thing_Create :: proc(vertices: [dynamic]f32, triangles: [dynamic] u32, colour: [3] f32) -> Thing {
    pos_vbo : u32
    gl.CreateBuffers(1, &pos_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, pos_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, len(vertices) * size_of(f32), &vertices[0], gl.STATIC_DRAW)

    colours := make([dynamic]f32, len(vertices))
    for i in 0..<len(colours)/3 {
        colours[i*3] = colour[0]; colours[i*3+1] = colour[1]; colours[i*3+2] = colour[2];
    }

    col_vbo : u32
    gl.CreateBuffers(1, &col_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, col_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, len(colours) * size_of(f32), &colours[0], gl.STATIC_DRAW)

    ebo : u32
    gl.CreateBuffers(1, &ebo)
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo)
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, len(triangles) * size_of(u32), &triangles[0], gl.STATIC_DRAW)

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

    return Thing {
        pos_vbo = pos_vbo,
        col_vbo = col_vbo,
        ebo = ebo,
        vao = vao,
        vertices = vertices,
        triangles = triangles,
        colours = colours
    }
}

Thing_Rotate :: proc(thing: ^Thing, R: matrix[4,4] f64 ) {
}

Thing_Draw :: proc(thing: ^Thing) {
    gl.BindVertexArray(thing.vao)
    gl.DrawElements(gl.TRIANGLES, size_of(thing.triangles) / size_of(u32), gl.UNSIGNED_INT, nil)
    gl.BindVertexArray(0)
}
