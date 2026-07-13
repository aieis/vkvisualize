use crate::{geometry::vec3::Vec3, utils::colours::{BLUE, CYAN, GREEN, RED, VIOLET, YELLOW}};

use super::mesh::Mesh;


pub fn make_prism(location: Vec3, dimensions: Vec3, col: Vec3) -> Mesh
{
    let lx   = dimensions.x;
    let ly   = dimensions.y;
    let lz   = dimensions.z;

    let x0  = location.x - lx / 2.0;
    let y0  = location.y - ly / 2.0;
    let z0  = location.z - lz / 2.0;

    let vertices = vec![
        // back
        Vec3::new(x0, y0, z0), Vec3::new(x0+lx, y0, z0), Vec3::new(x0+lx, y0+ly, z0), Vec3::new(x0,  y0+ly, z0),

        // front
        Vec3::new(x0, y0, z0+lz), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0,  y0+ly, z0+lz),

        // left
        Vec3::new(x0, y0, z0), Vec3::new(x0, y0, z0+lz), Vec3::new(x0, y0+ly, z0+lz), Vec3::new(x0, y0+ly, z0),

        // right
        Vec3::new(x0+lx, y0, z0), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0),

        // top
        Vec3::new(x0, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0), Vec3::new(x0, y0+ly, z0),

        // bottom
        Vec3::new(x0, y0, z0+lz), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0, z0), Vec3::new(x0, y0, z0),

    ];

    println!("{} {} {} {}", vertices[16], vertices[17], vertices[18], vertices[19]);

    // let colour = vec![
    //     col, col, col, col,
    //     col, col, col, col,
    //     col, col, col, col,
    //     col, col, col, col,
    //     col, col, col, col,
    //     col, col, col, col,
    // ];

    let colour = vec![
        RED, RED, RED, RED,
        GREEN, GREEN, GREEN, GREEN,
        BLUE, BLUE, BLUE, BLUE,
        CYAN, CYAN, CYAN, CYAN,
        YELLOW, YELLOW, YELLOW, YELLOW,
        VIOLET, VIOLET, VIOLET, VIOLET,
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

    let center = location;

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
