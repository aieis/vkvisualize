package main

import "core:fmt"

SHADER_SIMPLE_VERT :: #load("../assets/shaders/simple.vert")
SHADER_SIMPLE_FRAG :: #load("../assets/shaders/simple.frag")


import gl "vendor:OpenGL"

Cube :: struct {
    position: [3] f32,
    pos_vbo: u32,
    col_vbo: u32,
    ebo: u32,
    vao: u32,
    vertices: [12 * 6]f32,
    pos: [12 * 6]f32,
    indices: [6 * 6]u32,
    colours: [12 * 6]f32
}

Cube_Create :: proc(dim: f32) -> Cube {
    position := [3]f32 {0, 0, -1}
    ox := position[0]; oy := position[1]; oz := position[2];

    vertices := [72]f32 {
        -dim+ox, -dim+oy, -dim+oz, +dim+ox, -dim+oy, -dim+oz, +dim+ox, +dim+oy, -dim+oz, -dim+ox, +dim+oy, -dim+oz,
        -dim+ox, -dim+oy, +dim+oz, +dim+ox, -dim+oy, +dim+oz, +dim+ox, +dim+oy, +dim+oz, -dim+ox, +dim+oy, +dim+oz,
        -dim+ox, -dim+oy, -dim+oz, +dim+ox, -dim+oy, -dim+oz, +dim+ox, -dim+oy, +dim+oz, -dim+ox, -dim+oy, +dim+oz,
        -dim+ox, +dim+oy, -dim+oz, +dim+ox, +dim+oy, -dim+oz, +dim+ox, +dim+oy, +dim+oz, -dim+ox, +dim+oy, +dim+oz,
        -dim+ox, -dim+oy, -dim+oz, -dim+ox, -dim+oy, +dim+oz, -dim+ox, +dim+oy, +dim+oz, -dim+ox, +dim+oy, -dim+oz,
        +dim+ox, -dim+oy, -dim+oz, +dim+ox, -dim+oy, +dim+oz, +dim+ox, +dim+oy, +dim+oz, +dim+ox, +dim+oy, -dim+oz
    }

    colours := [72] f32 {
        0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
        1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5,
        1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5,
        0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5,
        0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5, 0.5, 1.0,
        1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0,
    }

    indices := [6 * 6]u32 {
        0+00,1+00,2+00,0+00,3+00,2+00,
        0+04,1+04,2+04,0+04,3+04,2+04,
        0+08,1+08,2+08,0+08,3+08,2+08,
        0+12,1+12,2+12,0+12,3+12,2+12,
        0+16,1+16,2+16,0+16,3+16,2+16,
        0+20,1+20,2+20,0+20,3+20,2+20,
    }

    pos_vbo : u32
    gl.CreateBuffers(1, &pos_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, pos_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, len(vertices) * size_of(f32), &vertices, gl.STATIC_DRAW)

    col_vbo : u32
    gl.CreateBuffers(1, &col_vbo)
    gl.BindBuffer(gl.ARRAY_BUFFER, col_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, len(colours) * size_of(f32), &colours, gl.STATIC_DRAW)

    ebo : u32
    gl.CreateBuffers(1, &ebo)
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo)
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, len(indices) * size_of(u32), &indices, gl.STATIC_DRAW)

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
        position = position,
        pos_vbo = pos_vbo,
        col_vbo = col_vbo,
        ebo = ebo,
        vao = vao,
        vertices = vertices,
        pos = vertices,
        indices = indices,
        colours = colours
    }
}

Cube_Draw :: proc(cube: ^Cube) {
    gl.BindVertexArray(cube.vao)
    gl.DrawElements(gl.TRIANGLES, size_of(cube.indices) / size_of(u32), gl.UNSIGNED_INT, nil)
    gl.BindVertexArray(0)
}

Cube_Rotate :: proc(cube: ^Cube, R: matrix[4,4] f64 ) {
    ox := cube.position[0]; oy := cube.position[1]; oz := cube.position[2];
    for i :=0 ; i < len(cube.vertices); i += 3 {
        x := (cube.vertices[i]-ox) * cast(f32) R[0][0] + (cube.vertices[i+1]-oy) * cast(f32) R[0][1] + (cube.vertices[i+2]-oz) * cast(f32) R[0][2]
        y := (cube.vertices[i]-ox) * cast(f32) R[1][0] + (cube.vertices[i+1]-oy) * cast(f32) R[1][1] + (cube.vertices[i+2]-oz) * cast(f32) R[1][2]
        z := (cube.vertices[i]-ox) * cast(f32) R[2][0] + (cube.vertices[i+1]-oy) * cast(f32) R[2][1] + (cube.vertices[i+2]-oz) * cast(f32) R[2][2]
        cube.pos[i + 0] = x+ox
        cube.pos[i + 1] = y+oy
        cube.pos[i + 2] = z+oz
    }
    x := cube.vertices[0]; y := cube.vertices[1]; z := cube.vertices[2]
    xp := cube.pos[0]; yp := cube.pos[1]; zp := cube.pos[2]
    fmt.println("Point 0: (", x, y, z, ") => (", xp, yp, zp, ")")
    

    gl.BindBuffer(gl.ARRAY_BUFFER, cube.pos_vbo)
    gl.BufferData(gl.ARRAY_BUFFER, len(cube.pos) * size_of(f32), &cube.pos, gl.STATIC_DRAW)

}
