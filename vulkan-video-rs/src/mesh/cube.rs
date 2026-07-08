use crate::geometry::vec3::Vec3;

use super::mesh::Mesh;


// Too low res for now
pub fn make_cube(x: f32, y: f32, z: f32, length: f32, col: [f32; 3]) -> Mesh
{
    let l   = length;
    let l_2 = l / 2.0;
    let x0  = x - l_2;
    let y0  = y - l_2;
    let z0  = z - l_2;

    let col = Vec3::new(col[0], col[1], col[2]);

    let vertices = vec![
        // back
        Vec3::new(x0, y0, z0), Vec3::new(x0+l, y0, z0), Vec3::new(x0+l, y0+l, z0), Vec3::new(x0,  y0+l, z0),

        // front
        Vec3::new(x0, y0, z0+l), Vec3::new(x0+l, y0, z0+l), Vec3::new(x0+l, y0+l, z0+l), Vec3::new(x0,  y0+l, z0+l),

        // left
        Vec3::new(x0, y0, z0), Vec3::new(x0, y0, z0+l), Vec3::new(x0, y0+l, z0+l), Vec3::new(x0, y0+l, z0),

        // right
        Vec3::new(x0+l, y0, z0), Vec3::new(x0+l, y0, z0+l), Vec3::new(x0+l, y0+l, z0+l), Vec3::new(x0+l, y0+l, z0),

        // top
        Vec3::new(x0, y0+l, z0+l), Vec3::new(x0+l, y0+l, z0+l), Vec3::new(x0+l, y0+l, z0), Vec3::new(x0, y0+l, z0),

        // bottom
        Vec3::new(x0, y0, z0+l), Vec3::new(x0+l, y0, z0+l), Vec3::new(x0+l, y0, z0), Vec3::new(x0, y0, z0),

    ];

    let colour = vec![
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
    ];

    let indices = vec![
        // back
        0, 2, 1, 0, 3, 2,

        // front
        4, 5, 6, 4, 6, 7,

        // left
        8, 9, 10, 8, 10, 11,

        // right
        12, 14, 13, 12, 15, 14,

        // top
        16, 17, 18, 16, 18, 19,

        // bottom
        20, 22, 21, 20, 23, 22,
    ];

    let center = Vec3::new(x, y, z);

    let normals = Mesh::create_normals(&vertices, &indices);

    Mesh {
        center,
        vertices,
        colour,
        indices,
        normals,
        dirty_vertices: true,
        dirty_colour: true,
        dirty_indices: true,
        dirty_normals: true,
    }
}
