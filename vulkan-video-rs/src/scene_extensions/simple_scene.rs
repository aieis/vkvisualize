use std::time::Instant;

use ash::vk;

use crate::geometry::vec3::Vec3;
use crate::mesh::prism;
use crate::rhi::allocator::{Allocator, BufferType};
use crate::vk_bundles::{BufferBundle, DeviceBundle};
use crate::{drawable::drawable_mesh::DrawableMesh, mesh::cube, vk_base::VkBase};
use crate::shader::{ShaderMesh, ShaderSpecialMesh};

pub struct SimpleScene
{
    pub time            : Instant,

    pub static_meshes  : Vec<DrawableMesh>,
    pub dynamic_meshes : Vec<DrawableMesh>,


    staging: BufferBundle,
    uniform: BufferBundle,
    descriptor_sets: Vec<vk::DescriptorSet>,
    going_left: bool,
    translation_amount: f32,
}

impl SimpleScene
{
    pub fn new(base: &VkBase, allocator: &mut Allocator) -> SimpleScene {

        let floor = prism::make_prism(Vec3::new(0.0, -5.0, 0.0), Vec3::new(20.0, 1.0, 20.0), Vec3::of(0.2));

        let prism_b = prism::make_debug_prism(Vec3::new(0.0, 0.0, -5.0), Vec3::new(2.0, 3.0, 8.0));

        let mut cube_c = prism::make_prism(Vec3::new(5.0, 0.0, 0.0), Vec3::of(0.5), Vec3::new(0.0, 0.0, 1.0));
        cube_c.rotate_x(45_f32.to_radians());
        cube_c.rotate_z(45_f32.to_radians());
        cube_c.recompute_normals();

        let cube_d = prism::make_prism(Vec3::new(-5.0, 0.0, 0.0), Vec3::of(0.5), Vec3::new(10.0, 0.0, 0.0));
        let cube_e = prism::make_debug_prism(Vec3::new(0.0, 0.0, 30.0), Vec3::of(5.0));


        let static_meshes = vec![
            DrawableMesh::new(&base.device, floor),
        ];

        let dynamic_meshes = vec![
            DrawableMesh::new(&base.device, prism_b),
            DrawableMesh::new(&base.device, cube_c),
            DrawableMesh::new(&base.device, cube_d),
            DrawableMesh::new(&base.device, cube_e),
        ];

        let staging = allocator.alloc(BufferType::Staging, std::mem::size_of::<f32>() as u64 * 2).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, std::mem::size_of::<f32>() as u64 * 2).unwrap();

        let descriptor_sets = Self::create_descriptor_sets(&base.device, base.descriptor_pool, base.graphics_pipelines[ShaderSpecialMesh::ID].ubo.as_ref().unwrap()[0], &uniform, base.max_in_flight);

        let time = Instant::now();


        Self {
            time,
            static_meshes,
            dynamic_meshes,
            staging,
            uniform,
            descriptor_sets,
            going_left: false,
            translation_amount: 0.0,
        }
    }

    pub fn update(base: &VkBase, cb: &vk::CommandBuffer, scenes: &mut [SimpleScene], aspect_ratio: f32) {
        for scene in scenes.iter_mut() {

            let mut v = 1e-2;

            const TRANSLATION_MAX: f32 = 1.0;
            if scene.translation_amount >= TRANSLATION_MAX {
                scene.going_left = !scene.going_left;
                scene.translation_amount = 0.0;
            } else {
                scene.translation_amount += v;
                v *= (TRANSLATION_MAX - scene.translation_amount) / TRANSLATION_MAX;
            }

            let d = Vec3::Y * 0.5;

            let v = if scene.going_left { d * -v } else { d * v };

            for mesh in scene.dynamic_meshes.iter_mut() {
                mesh.mesh.translate(v);
                mesh.mesh.rotate_y(1e-2);
                mesh.mesh.recompute_normals();
            }



            DrawableMesh::update(&base.device, &cb, &mut scene.dynamic_meshes);
            DrawableMesh::update(&base.device, &cb, &mut scene.static_meshes);


            let mut et = [scene.time.elapsed().as_secs_f32(), aspect_ratio];

            unsafe {
                let data_ptr = base.device.logical.map_memory(scene.staging.memory, scene.staging.offset, scene.staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut f32;
                data_ptr.copy_from_nonoverlapping(et.as_mut_ptr(), scene.staging.size as usize);
                base.device.logical.unmap_memory(scene.staging.memory);

                let copy_region = [
                    vk::BufferCopy::default()
                        .src_offset(scene.staging.offset)
                        .dst_offset(scene.uniform.offset)
                        .size(scene.staging.size)
                ];

                base.device.logical.cmd_copy_buffer(*cb, scene.staging.buffer, scene.uniform.buffer, &copy_region);
            }
        }

    }

    pub fn draw(base: &mut VkBase, cb: &vk::CommandBuffer, scenes: &[SimpleScene], current_image: usize) {

        let pso = &base.graphics_pipelines[ShaderSpecialMesh::ID];

        unsafe {
            base.device.logical.cmd_bind_pipeline(*cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);
        }

        for scene in scenes {
            unsafe {
                base.device.logical.cmd_bind_descriptor_sets(
                    *cb, vk::PipelineBindPoint::GRAPHICS, pso.layout, 0,
                    &scene.descriptor_sets[current_image..current_image+1], &[]);
            }

            DrawableMesh::draw(&base.device, cb, pso, &scene.static_meshes);
            DrawableMesh::draw(&base.device, cb, pso, &scene.dynamic_meshes);
        }
    }

    fn create_descriptor_sets(
        device: &DeviceBundle,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        buffer: &BufferBundle,
        swapchain_images_size: usize,
    ) -> Vec<vk::DescriptorSet> {
        let mut layouts: Vec<vk::DescriptorSetLayout> = vec![];
        for _ in 0..swapchain_images_size {
            layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.logical.allocate_descriptor_sets(&descriptor_set_allocate_info).unwrap() };

        for &descriptor_set in descriptor_sets.iter() {
            let descriptor_image_infos = [vk::DescriptorBufferInfo {
                buffer: buffer.buffer,
                offset: buffer.offset,
                range: buffer.size,
            }];

            let descriptor_write_sets = [
                vk::WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&descriptor_image_infos)
            ];

            unsafe {
                device.logical.update_descriptor_sets(&descriptor_write_sets, &[]);
            }
        }

        descriptor_sets

    }


    pub fn release(base: &VkBase, scenes: &mut [Self]) {
        for scene in scenes.iter_mut() {
            DrawableMesh::release(&base.device, &mut scene.dynamic_meshes);
            scene.dynamic_meshes.clear();
            DrawableMesh::release(&base.device, &mut scene.static_meshes);
            scene.static_meshes.clear();
        }
    }

}
