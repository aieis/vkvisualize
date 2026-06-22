use ash::vk;

use crate::{drawable::drawable_mesh::DrawableMesh, mesh::cube, vk_base::VkBase};

pub struct SimpleScene
{
    pub mesh_shader_idx : usize,
    pub mesh_bundles    : Vec<DrawableMesh>,
}

impl SimpleScene
{
    pub fn new(base: &VkBase, mesh_shader_idx : usize) -> SimpleScene {
        let mesh_bundles = vec![
            DrawableMesh::new(&base.device, cube::make_cube(0.75, 0.75, 0.25, 0.5, [0.0, 1.0, 0.0]))
        ];

        Self {
            mesh_shader_idx,
            mesh_bundles,
        }
    }

    pub fn update(base: &VkBase, cb: &vk::CommandBuffer, scenes: &mut [SimpleScene]) {
        for scene in scenes.iter_mut() {

            for mesh in scene.mesh_bundles.iter_mut() {
                mesh.mesh.rotate_z(1e-3);
            }

            DrawableMesh::update(&base.device, &cb, &mut scene.mesh_bundles);
        }

    }

    pub fn draw(base: &mut VkBase, cb: &vk::CommandBuffer, scenes: &[SimpleScene]) {
        for scene in scenes {
            DrawableMesh::draw(&base.device, cb, &base.graphics_pipelines[scene.mesh_shader_idx], &scene.mesh_bundles);
        }
    }

    pub fn release(base: &VkBase, scenes: &mut [Self]) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.mesh_bundles);
            scene.mesh_bundles.clear();
        }
    }

}
