package mesh;

import "../utils"

Cone :: struct {
    radius: f32,
    height: f32,
    vertices: [dynamic] f32,
    triangles: [dynamic] u32,
}


Cone_Create :: proc(radius: f32, height: f32) -> Cone {
    eighth_segments : u32 : 36
    circle := utils.Circle_Create(0, 0, radius, eighth_segments)
    defer delete(circle)


    vertices := make([dynamic] f32, (eighth_segments * 8 + 1) * 3 ) // vertices on circle + vertex at the top
    triangles := make([dynamic] u32, (eighth_segments * 8 * 2) * 3) // triangels on the circle + triangles to the circle

    N := eighth_segments * 8
    vertices[N*3+0] = 0
    vertices[N*3+1] = height
    vertices[N*3+2] = 0

    for i in 0..< N {
        idx := i * 3
        cidx := i * 2
        vertices[idx+0] = circle[cidx + 0]
        vertices[idx+1] = 0
        vertices[idx+2] = circle[cidx + 1]

        tidx := i * 6
        // triangles at the circle
        triangles[tidx + 0] = 0
        triangles[tidx + 1] = i
        triangles[tidx + 2] = (i + 1) % N

        //triangles to the top vertex
        triangles[tidx + 3] = N
        triangles[tidx + 4] = i
        triangles[tidx + 5] = (i + 1) % N
    }

    return Cone {
        radius = radius,
        height = height,
        vertices = vertices,
        triangles = triangles,
    }
}

Cone_Delete :: proc(cone: ^Cone) {
    delete(cone.triangles)
    delete(cone.vertices)
}
