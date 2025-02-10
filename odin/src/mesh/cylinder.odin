package mesh;

import "../utils"

Cylinder :: struct {
    radius: f32,
    height: f32,
    vertices: [dynamic] f32,
    triangles: [dynamic] u32,
}


Cylinder_Create :: proc(radius: f32, height: f32) -> Cylinder {
    eighth_segments : u32 : 36
    circle := utils.Circle_Create(0, 0, radius, eighth_segments)
    defer delete(circle)


    N := eighth_segments * 8

    vertices := make([dynamic] f32, N * 2 * 3)  // vertices on bottom + vertices on top
    triangles := make([dynamic] u32, N * 4 * 3) // triangels on the circles + triangles between the circles

    for i in 0..< N {
        idx := i * 3
        cidx := i * 2
        vertices[idx+0] = circle[cidx + 0]
        vertices[idx+1] = 0
        vertices[idx+2] = circle[cidx + 1]

        vertices[N*3+idx+0] = circle[cidx + 0]
        vertices[N*3+idx+1] = height
        vertices[N*3+idx+2] = circle[cidx + 1]

        tidx := i * 12
        // triangles at the bases
        triangles[tidx + 0] = 0
        triangles[tidx + 1] = i
        triangles[tidx + 2] = (i + 1) % N

        triangles[tidx + 3] = N
        triangles[tidx + 4] = N + i
        triangles[tidx + 5] = N + (i + 1) % N

        //triangles between the bases 
        triangles[tidx + 6] = i
        triangles[tidx + 8] = N + (i + 1) % N
        triangles[tidx + 7] = N + i
        
        triangles[tidx + 9] = i
        triangles[tidx + 10] = N + (i + 1) % N
        triangles[tidx + 11] = (i + 1) % N
    }

    return Cylinder {
        radius = radius,
        height = height,
        vertices = vertices,
        triangles = triangles,
    }
}

Cylinder_Delete :: proc(obj: ^Cylinder) {
    delete(obj.triangles)
    delete(obj.vertices)
}
