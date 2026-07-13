use std::time::Instant;

use ash::vk;

use crate::geometry::vec3::Vec3;
use crate::mesh::prism;
use crate::{drawable::drawable_mesh::DrawableMesh, mesh::cube, vk_base::VkBase};
use crate::shader::{ShaderMesh, ShaderSpecialMesh};

pub struct SimpleScene
{
    pub time            : Instant,
    pub mesh_bundles    : Vec<DrawableMesh>,
}

impl SimpleScene
{
    pub fn new(base: &VkBase) -> SimpleScene {

        // let mut cube_a = cube::make_cube(0.0, -12.0, 0.0, 20.0, [0.0, 0.0, 0.2]);
        // cube_a.rotate_x(85_f32.to_radians());
        // cube_a.recompute_normals();

        let mut prism_b = prism::make_prism(Vec3::of(0.0), Vec3::new(0.75, 0.5, 1.0), Vec3::new(0.0, 1.0, 0.0));
        // prism_b.rotate_z(45_f32.to_radians());
        // prism_b.rotate_y(60_f32.to_radians());
        // prism_b.recompute_normals();


        // let mut cube_c = cube::make_cube(1.0, 0.0, 0.0, 0.5, [0.0, 0.1, 0.1]);
        // // cube_c.rotate_x(20_f32.to_radians());
        // // cube_c.rotate_x(85_f32.to_radians());

        // let mut cube_d = cube::make_cube(0.0, 0.0, -1.0, 0.5, [0.0, 0.2, 0.2]);



        let mesh_bundles = vec![
            // DrawableMesh::new(&base.device, cube_a),
            DrawableMesh::new(&base.device, prism_b),
            // DrawableMesh::new(&base.device, cube_c),
            // DrawableMesh::new(&base.device, cube_d),
        ];

        let time = Instant::now();

        Self {
            time,
            mesh_bundles
        }
    }

    pub fn update(base: &VkBase, cb: &vk::CommandBuffer, scenes: &mut [SimpleScene]) {
        for scene in scenes.iter_mut() {

            for mesh in scene.mesh_bundles.iter_mut() {
                mesh.mesh.rotate_z(1e-3);
                mesh.mesh.rotate_y(1e-2);
                // mesh.mesh.rotate_x(1e-2);
                mesh.mesh.recompute_normals();

                // let g = mesh.mesh.colour[0].y;
                // let b = mesh.mesh.colour[0].z;
                // let et = scene.time.elapsed().as_secs_f32();
                // mesh.mesh.set_colour([et, g, b]);
            }



            DrawableMesh::update(&base.device, &cb, &mut scene.mesh_bundles);
        }

    }

    pub fn draw(base: &mut VkBase, cb: &vk::CommandBuffer, scenes: &[SimpleScene]) {
        for scene in scenes {
            DrawableMesh::draw(&base.device, cb, &base.graphics_pipelines[ShaderSpecialMesh::ID], &scene.mesh_bundles);
        }
    }

    pub fn release(base: &VkBase, scenes: &mut [Self]) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.mesh_bundles);
            scene.mesh_bundles.clear();
        }
    }

}
