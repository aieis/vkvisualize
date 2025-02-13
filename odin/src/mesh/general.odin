package mesh

import "core:math/linalg"
import "core:fmt"

Mesh :: struct {
    vertices: [dynamic]f32,
    triangles: [dynamic]u32,
    colours: [dynamic]f32
}

/* Lines are of the form of the from vertex to the to vertex like below:
 * [v1_fx, v1_fy, v1_fz, v1_tx, v1_ty, v1_tz, ...] */
Mesh_FromLines :: proc(lines: []f32, thickness: f32, submesh_offset: u32 = 0) -> Mesh{
    N := (len(lines) / 3) / 2

    vertices := make([dynamic]f32, N * 8 * 3)
    triangles := make([dynamic]u32, N * 12 * 3)

    t2 := thickness / 2
    up := [3]f32{0, 1, 0}

    line_triangles := [36]u32 {
        0, 2, 1,    0, 2, 3,    0, 5, 4,    0, 5, 1,    0, 7, 3,   0, 7, 4,
        6, 4, 5,    6, 4, 7,    6, 1, 2,    6, 1, 5,    6, 3, 7,   6, 3, 2
    }

    for i in 0..< N {
        idx := i*6

        pf := [3]f32{lines[idx]  , lines[idx+1], lines[idx+2]}
        pt := [3]f32{lines[idx+3], lines[idx+4], lines[idx+5]}

        fvu := pt - pf
        fv := linalg.normalize(fvu)
        lv := linalg.normalize(linalg.cross(up, fv))
        uv := linalg.normalize(linalg.cross(fv, lv))

        if fv == up {
            lv = {-1, 0, 0}
            uv = {0, 0, 1}
        }

        vidx := i*8*3
        pts := [8][3]f32{
            pf + lv*t2 + uv*t2, pf - lv*t2 + uv*t2, pf - lv*t2 - uv*t2, pf + lv*t2 - uv*t2,  // left face
            pt + lv*t2 + uv*t2, pt - lv*t2 + uv*t2, pt - lv*t2 - uv*t2, pt + lv*t2 - uv*t2   // right face
        }

        for ofs, pts_idx in pts {
            vidx := i*24 + 3*pts_idx
            vertices[vidx+0] = ofs[0]; vertices[vidx+1] = ofs[1]; vertices[vidx+2] = ofs[2]
        }

        tidx := i * 36
        for v, nt_idx in line_triangles {
            triangles[tidx+nt_idx] = v + cast(u32) vidx / 3 + submesh_offset
        }
    }

    return Mesh {
        vertices = vertices,
        triangles = triangles,
        colours = make([dynamic]f32, 0)
    }
}
