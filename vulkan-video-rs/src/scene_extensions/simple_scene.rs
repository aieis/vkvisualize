use ash::vk;

use crate::{drawable::drawable_mesh::DrawableMesh, mesh::cube, vk_base::VkBase};
use crate::shader::ShaderMesh;

pub struct SimpleScene
{
    pub mesh_bundles    : Vec<DrawableMesh>,
}

impl SimpleScene
{
    pub fn new(base: &VkBase) -> SimpleScene {

        let mut cube = cube::make_cube(0.75, 0.75, 0.25, 0.5, [0.0, 1.0, 0.0]);

        let mesh_bundles = vec![
            DrawableMesh::new(&base.device, cube)
        ];

        Self {
            mesh_bundles
        }
    }

    pub fn update(base: &VkBase, cb: &vk::CommandBuffer, scenes: &mut [SimpleScene]) {
        for scene in scenes.iter_mut() {

            for mesh in scene.mesh_bundles.iter_mut() {
                mesh.mesh.rotate_z(1e-3);
                mesh.mesh.rotate_y(1e-3);
            }

            DrawableMesh::update(&base.device, &cb, &mut scene.mesh_bundles);
        }

    }

    pub fn draw(base: &mut VkBase, cb: &vk::CommandBuffer, scenes: &[SimpleScene]) {
        for scene in scenes {
            DrawableMesh::draw(&base.device, cb, &base.graphics_pipelines[ShaderMesh::ID], &scene.mesh_bundles);
        }
    }

    pub fn release(base: &VkBase, scenes: &mut [Self]) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.mesh_bundles);
            scene.mesh_bundles.clear();
        }
    }

}
