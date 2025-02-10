package mesh;

import "core:fmt"

import "../utils"

Cone :: struct {
    radius: f32,
    height: f32,
    vertices: [dynamic] f32,
    triangles: [dynamic] u32,
}


Cone_Create :: proc(radius: f32, height: f32) -> Cone {
    quarter_segments : u32 : 100
    circle := utils.Circle_Create(0, -10, radius, quarter_segments)
    defer delete(circle)


    vertices := make([dynamic] f32, (quarter_segments * 4 + 1) * 3 ) // vertices on circle + vertex at the top
    triangles := make([dynamic] u32, (quarter_segments * (4 + 4)) * 3) // triangels on the circle + triangles to the circle

    vertices[0] = 0
    vertices[1] = height
    vertices[2] = 0

    for i in 0..<quarter_segments {
        idx := i * 12 + 3
        cidx := i * 8
        vertices[idx+0] = circle[cidx + 0]
        vertices[idx+1] = 0
        vertices[idx+2] = circle[cidx + 1]
        vertices[idx+3] = circle[cidx + 2]
        vertices[idx+4] = 0
        vertices[idx+5] = circle[cidx + 3]
        vertices[idx+6] = circle[cidx + 4]
        vertices[idx+7] = 0
        vertices[idx+8] = circle[cidx + 5]
        vertices[idx+9] = circle[cidx + 6]
        vertices[idx+10] = 0
        vertices[idx+11] = circle[cidx + 7]

        tidx := i * 24
        // triangles at the circle
        triangles[tidx + 0] = 1
        triangles[tidx + 1] = i
        triangles[tidx + 2] = i + 4
        triangles[tidx + 3] = 1
        triangles[tidx + 4] = i + 1
        triangles[tidx + 5] = i + 1 + 4
        triangles[tidx + 6] = 1
        triangles[tidx + 7] = i + 2
        triangles[tidx + 8] = i + 2 + 4
        triangles[tidx + 9] = 1
        triangles[tidx + 10] = i + 3
        triangles[tidx + 11] = i + 3 + 4

        //triangles to the top vertex
        triangles[tidx + 12] = 0
        triangles[tidx + 13] = i
        triangles[tidx + 14] = i + 4

        triangles[tidx + 15] = 0
        triangles[tidx + 16] = i + 1
        triangles[tidx + 17] = i + 1 + 4

        triangles[tidx + 18] = 0
        triangles[tidx + 19] = i + 2
        triangles[tidx + 20] = i + 2 + 4

        triangles[tidx + 21] = 0
        triangles[tidx + 22] = i + 3
        triangles[tidx + 23] = i + 3 + 4        
    }

    fmt.println(vertices)
    fmt.println(triangles)

    return Cone {
        radius = radius,
        height = height,
        vertices = vertices,
        triangles = triangles,
    }
}
