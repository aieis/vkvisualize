package mesh

import "../utils"

Arrow :: struct {
    length: f32,
    vertices: [dynamic] f32,
    triangles: [dynamic] u32,
}


Arrow_Create :: proc(length: f32) -> Arrow {
    radius := length / 6
    head := Cone_Create(radius, length/3)
    defer Cone_Delete(&head)

    shaft := Cylinder_Create(radius/2, length*2/3)
    defer Cylinder_Delete(&shaft)

    H_N := len(head.vertices)
    H_TN := len(head.triangles)
    H_VN := cast(u32) H_N/3

    S_N := len(shaft.vertices)
    S_TN := len(shaft.triangles)

    vertices := make([dynamic] f32, H_N + S_N)
    triangles := make([dynamic] u32, H_TN + S_TN)

    for i in 0..< H_VN {
        vertices[i*3] = head.vertices[i*3]
        vertices[i*3+1] = head.vertices[i*3+1]+length*2/3
        vertices[i*3+2] = head.vertices[i*3+2]
    }

    for i in 0..< S_N {
        vertices[H_N+i] = shaft.vertices[i]
    }

    for i in 0..< H_TN {
        triangles[i] = head.triangles[i]
    }

    for i in 0..< S_TN {
        triangles[H_TN + i] = shaft.triangles[i] + H_VN
    }

    return Arrow {
        length = length,
        vertices = vertices,
        triangles = triangles,
    }
}

Arrow_Delete :: proc(obj: ^Arrow) {
    delete(obj.triangles)
    delete(obj.vertices)
}
