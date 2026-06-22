use super::mesh::Mesh;

pub fn make_cube(x: f32, y: f32, z: f32, length: f32, col: [f32; 3]) -> Mesh
{
    let l   = length;
    let l_2 = l / 2.0;
    let x0  = x - l_2;
    let y0  = y - l_2;
    let z0  = z - l_2;

    let vertices = vec![
        // back
        [x0, y0, z0], [x0+l, y0, z0], [x0+l, y0+l, z0], [x0,  y0+l, z0],

        // front
        [x0, y0, z0+l], [x0+l, y0, z0+l], [x0+l, y0+l, z0+l], [x0,  y0+l, z0+l],
    ];

    let colour = vec![
        col, col, col, col,
        col, col, col, col
    ];

    let indices = vec![
        // front
        0, 1, 2, 0, 2, 3,

        // back
        0+4, 1+4, 2+4, 0+4, 2+4, 3+4
    ];

    let center = [x, y, z];

    Mesh {
        center,
        vertices,
        colour,
        indices,
        dirty_vertices: true,
        dirty_colour: true,
        dirty_indices: true,
    }
}
