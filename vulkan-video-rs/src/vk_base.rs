use std::ffi::{c_char, c_void, CStr, CString};

use ash::{ext::debug_utils, khr};

use crate::{drawable::drawable_common::DescSetBinding, shader::Shader};
use crate::vk_bundles::*;
use crate::drawable::drawable_common::PipelineDescriptor;

use ash::vk;
use winit::{raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};

pub struct VkBase {
    pub _entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_utils_loader: debug_utils::Instance,
    pub debug_messenger: vk::DebugUtilsMessengerEXT,
    pub surface: SurfaceBundle,
    pub device: DeviceBundle,
    pub swapchain: SwapchainBundle,
    pub image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub commands: Vec<CommandBundle>,
    pub spare_command: CommandBundle,
    pub descriptor_pool: vk::DescriptorPool,
    pub in_flight_buffers: Vec<(vk::CommandBuffer, vk::Fence)>,
    pub sync_objects: SyncObjectsBundle,
    pub current_frame: usize,
    pub is_framebuffer_resized: bool,
    pub window: Window,
    pub max_in_flight: usize,
}

impl VkBase {
    pub fn new(window: Window, max_in_flight: usize) -> Self {
        let (entry, instance) = VkBase::create_instance(&window);
        let (debug_utils_loader, debug_messenger) = VkBase::setup_validation(&entry, &instance);

        let surface = VkBase::create_surface(&entry, &instance, &window);
        let device = VkBase::select_phsyical_device(&instance, &surface);
        let swapchain = VkBase::create_swapchain(&instance, &device, &surface, &window);
        let image_views = VkBase::create_image_views(&device, &swapchain);
        let max_in_flight = if image_views.len() < max_in_flight { image_views.len() } else { max_in_flight };
	let render_pass = VkBase::create_render_pass(&device, &swapchain);
	let framebuffers = VkBase::create_framebuffers(&device, &render_pass, &image_views, &swapchain);
	let commands = VkBase::create_command_pools(&device, max_in_flight, 1);
        let spare_command = VkBase::create_command_pools(&device, 1, max_in_flight).remove(0);
        let sync_objects = VkBase::create_sync_objects(&device, max_in_flight);

        let descriptor_pool = VkBase::create_descriptor_pool(&device, swapchain.images.len());


        Self {
            _entry: entry,

            instance,
            debug_utils_loader,
            debug_messenger,

            surface,
            device,
            swapchain,
            image_views,
	    render_pass,

	    framebuffers,
            commands,
            spare_command,
            in_flight_buffers: vec![],

            descriptor_pool,

	    sync_objects,
	    current_frame: 0,
            is_framebuffer_resized: false,

            window,
            max_in_flight
        }
    }

    pub fn begin_renderpass_command_buffer(&self, command_buffer: &vk::CommandBuffer, framebuffer: &vk::Framebuffer) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
	    .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            self.device.logical.begin_command_buffer(*command_buffer, &command_buffer_begin_info)
                .expect("Failed to begin recording Command Buffer at beginning!");
        }

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(*framebuffer)
            .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.swapchain.extent})
            .clear_values(&clear_values);

        unsafe {
            self.device.logical.cmd_begin_render_pass(*command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
        }
    }


    pub fn end_command_buffer(&self, command_buffer: &vk::CommandBuffer) {
        let command_buffer = *command_buffer;
        unsafe {
            self.device.logical.cmd_end_render_pass(command_buffer);
            let _ = self.device.logical.end_command_buffer(command_buffer);
        }
    }

    pub fn create_graphics_pipeline(&self, pipeline_desc: PipelineDescriptor, shader: Box<dyn Shader>) -> GraphicsPipelineBundle {
        // TODO: ubo_set_layout should probably be created externally.
        let ubo = if pipeline_desc.ubo_layout_bindings.len() == 0 { None } else {
            Some(vec![Self::create_descriptor_set_layout(&self.device, &pipeline_desc.ubo_layout_bindings)])
        };

        return VkBase::create_graphics_pipeline_impl(&self.device, &self.swapchain, &self.render_pass, pipeline_desc, ubo, shader);
    }

    pub fn recreate_graphics_pipeline(&self, graphics_pipeline: GraphicsPipelineBundle) -> GraphicsPipelineBundle {
        return VkBase::create_graphics_pipeline_impl(&self.device, &self.swapchain, &self.render_pass, graphics_pipeline.pipeline_desc, graphics_pipeline.ubo, graphics_pipeline.shader);
    }

    pub fn recreate_swapchain(&mut self) {
        // parameters -------------
        unsafe { self.device.logical.device_wait_idle().expect("Failed to wait device idle!") };
        self.cleanup_swapchain();
        self.swapchain = VkBase::create_swapchain(&self.instance, &self.device, &self.surface, &self.window);
        self.image_views = VkBase::create_image_views(&self.device, &self.swapchain);
	self.render_pass = VkBase::create_render_pass(&self.device, &self.swapchain);
        self.framebuffers = VkBase::create_framebuffers(&self.device, &self.render_pass, &self.image_views, &self.swapchain);
        self.max_in_flight = if self.image_views.len() < self.max_in_flight { self.image_views.len() } else { self.max_in_flight };
    }

    pub fn cleanup_swapchain(&self) {
        unsafe {
            for &framebuffer in self.framebuffers.iter() {
                self.device.logical.destroy_framebuffer(framebuffer, None);
            }

            self.device.logical.destroy_render_pass(self.render_pass, None);
            for &image_view in self.image_views.iter() {
                self.device.logical.destroy_image_view(image_view, None);
            }

            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
        }
    }

    pub fn cleanup_in_flight_buffers(&mut self) {
        let len_orig = self.in_flight_buffers.len();
        for i in 0..self.in_flight_buffers.len() {
            let idx = len_orig-i-1;
            let (command_buffer, fence) = self.in_flight_buffers[idx];
            let fence_status =  unsafe {
                self.device.logical.get_fence_status(fence).expect("Getting fence status failed")
            };

            if fence_status {
                self.spare_command.buffers.push(command_buffer);
                self.sync_objects.spare_fences.push(fence);
                self.in_flight_buffers.remove(idx);
            }
        }
    }


    /* Misc vulkan */
    pub fn create_instance(window: &Window) -> (ash::Entry, ash::Instance) {
        // The entry contains the global vk functions
        let entry = unsafe { ash::Entry::load().unwrap() };

        // List all the supported layers
        let vk_layers = unsafe {entry.enumerate_instance_layer_properties().expect("Could not enumerate layers")};

        println!("Found {} layer(s).", vk_layers.len());
        for layer in vk_layers {
            println!("\t{:?}", layer.layer_name_as_c_str().unwrap())
        }
        println!();

        // Select layers to enable
        let layers = [c"VK_LAYER_KHRONOS_validation"];
        let layers_raw: Vec<*const c_char> = layers.iter().map(|raw_name| raw_name.as_ptr()).collect();

        // List all the supported extensions
        let vk_extensions = unsafe { entry.enumerate_instance_extension_properties(None).unwrap() };
        println!("Found {} extension(s).", vk_extensions.len());
        for extension in vk_extensions {
            println!("\t{:?}", extension.extension_name_as_c_str().unwrap())
        }
        println!();

        // Select the extensions
        let mut extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw()).unwrap().to_vec();
        extensions.push(debug_utils::NAME.as_ptr());

        // Create the instance
        let app_name = c"Vulkan Video";

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layers_raw)
            .enabled_extension_names(&extensions);

        let instance = unsafe { entry.create_instance(&create_info, None).expect("Failed to create instance.") };

        (entry, instance)
    }

    pub fn create_surface(entry: &ash::Entry, instance: &ash::Instance, window: &Window) -> SurfaceBundle {
        let display_handle = window.display_handle().unwrap();
        let window_handle = window.window_handle().unwrap();
        let surface = unsafe { ash_window::create_surface(entry, instance, display_handle.as_raw(), window_handle.as_raw(), None).unwrap() };
        let surface_loader = khr::surface::Instance::new(entry, instance);

        SurfaceBundle {
            surface,
            loader: surface_loader
        }
    }

    /* Select device */
    pub fn select_phsyical_device(instance: &ash::Instance, surface: &SurfaceBundle) -> DeviceBundle{
        let devs = unsafe { instance.enumerate_physical_devices().unwrap() };

        let mut queues = Vec::new();

        println!("Found {} device(s).", devs.len());
        for dev in devs.iter() {
            let properties = unsafe { instance.get_physical_device_properties(*dev) };
            let queue_props = unsafe { instance.get_physical_device_queue_family_properties(*dev) };
            let features = unsafe { instance.get_physical_device_features(*dev) };

            println!("\t{:?}", properties.device_name_as_c_str().unwrap());

            if !features.sampler_anisotropy == vk::FALSE {
                continue;
            }

            for (i, queue) in queue_props.iter().enumerate() {
                let surface_support = unsafe { surface.loader.get_physical_device_surface_support(*dev, i as u32, surface.surface).unwrap() };
                if surface_support && queue.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    queues.push((i as u32, *dev));
                }
            }

        }

        println!();

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queues[0].0)
            .queue_priorities(&[1.0]);

        let device_extension_names_raw = [
            khr::swapchain::NAME.as_ptr(),
        ];

        let physical_features = vk::PhysicalDeviceFeatures::default()
            .sampler_anisotropy(true);

        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_features(&physical_features)
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw);

        let physical = queues[0].1;
        let queue_family_index = queues[0].0 as u32;
        let device = unsafe { instance.create_device(physical, &device_create_info, None).unwrap() };
        let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical) };

        DeviceBundle {
            logical: device,
            physical,
            queue_family_index,
            present_queue,
            mem_properties
        }
    }

    /* Setup the swapchain */
    pub fn create_swapchain(instance: &ash::Instance, device: &DeviceBundle, surface: &SurfaceBundle, window: &Window) -> SwapchainBundle {

        let surface_format = unsafe { surface.loader.get_physical_device_surface_formats(device.physical, surface.surface).unwrap()[0] };

        let surface_capabilities = unsafe { surface.loader.get_physical_device_surface_capabilities(device.physical, surface.surface).unwrap() };
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let window_size = window.inner_size();

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => vk::Extent2D {
                width: window_size.width,
                height: window_size.height,
            },
            _ => surface_capabilities.current_extent,
        };

        let pre_transform = if surface_capabilities.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = unsafe { surface.loader.get_physical_device_surface_present_modes(device.physical, surface.surface).unwrap() };

        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let swapchain_loader = khr::swapchain::Device::new(&instance, &device.logical);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap() };
        let present_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

        SwapchainBundle {
            swapchain,
            loader: swapchain_loader,
            images: present_images,
            format: surface_format.format,
            extent: surface_resolution
        }
    }

    pub fn create_image_views(device: &DeviceBundle, swapchain: &SwapchainBundle) -> Vec<vk::ImageView>{
        let mut present_image_views: Vec<vk::ImageView> = Vec::new();
        let rgba_component = vk::ComponentMapping {
            r: vk::ComponentSwizzle::R,
            g: vk::ComponentSwizzle::G,
            b: vk::ComponentSwizzle::B,
            a: vk::ComponentSwizzle::A,
        };

        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        for image in swapchain.images.iter() {
            let create_view_info = vk::ImageViewCreateInfo::default()
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain.format)
                .components(rgba_component)
                .subresource_range(subresource_range)
                .image(*image);
            let view = unsafe { device.logical.create_image_view(&create_view_info, None).unwrap() };
            present_image_views.push(view);
        }

        present_image_views
    }

    /* Setup the graphics pipeline */
    pub fn create_graphics_pipeline_impl(device: &DeviceBundle, swapchain: &SwapchainBundle, renderpass: &vk::RenderPass, pipeline_desc: PipelineDescriptor, ubo: Option<Vec<vk::DescriptorSetLayout>>, shader: Box<dyn Shader>) -> GraphicsPipelineBundle {
	let (shader_vertex, shader_fragment) = shader.compile(&device.logical);
        let main_function_name = CString::new("main").unwrap(); // the beginning function name in shader code.
	let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::default()
	    .name(&main_function_name)
	    .stage(vk::ShaderStageFlags::VERTEX)
	    .module(shader_vertex);

	let fragment_shader_stage = vk::PipelineShaderStageCreateInfo::default()
	    .name(&main_function_name)
	    .stage(vk::ShaderStageFlags::FRAGMENT)
	    .module(shader_fragment);

	let shader_stage_create_infos = [vertex_shader_stage, fragment_shader_stage];

        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&pipeline_desc.vertex_bindings)
            .vertex_attribute_descriptions(&pipeline_desc.vertex_attributes);

	let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo::default()
	    .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
	    .primitive_restart_enable(false);

	let viewports = [vk::Viewport {
	    x: 0.0,
	    y: 0.0,
	    width: swapchain.extent.width as f32,
	    height: swapchain.extent.height as f32,
	    min_depth: 0.0,
	    max_depth: 1.0
	}];

	let scissors = [vk::Rect2D {
	    offset: vk::Offset2D {x: 0, y: 0},
	    extent: swapchain.extent
	}];

	// let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
	let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default();
	//.dynamic_states(&dynamic_states);

	let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
	    .viewports(&viewports)
	    .scissors(&scissors);

	let rasterization_info = vk::PipelineRasterizationStateCreateInfo::default()
	    .cull_mode(vk::CullModeFlags::BACK)
	    .front_face(vk::FrontFace::CLOCKWISE)
	    .polygon_mode(vk::PolygonMode::FILL)
	    .depth_clamp_enable(false)
	    .rasterizer_discard_enable(false)
	    .line_width(1.0);

	let multisample_state_info = vk::PipelineMultisampleStateCreateInfo::default()
	    .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let depth_state_create_info = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false)
            .front(stencil_state)
            .back(stencil_state)
            .max_depth_bounds(1.0)
            .min_depth_bounds(0.0);


        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

	let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

	let mut layout_create_info = vk::PipelineLayoutCreateInfo::default();
        if let Some(ubo) = ubo.as_ref() {
            layout_create_info = layout_create_info.set_layouts(&ubo);
        }

        let pipeline_layout = unsafe { device.logical.create_pipeline_layout(&layout_create_info, None).unwrap() };

        let graphic_pipeline_infos = [vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_create_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(*renderpass)];

        let graphics_pipelines = unsafe {
            device.logical.create_graphics_pipelines(vk::PipelineCache::null(), &graphic_pipeline_infos, None)
                .expect("Failed to create Graphics Pipeline!.")
        };

        unsafe {
            device.logical.destroy_shader_module(shader_vertex, None);
            device.logical.destroy_shader_module(shader_fragment, None);
        }


        GraphicsPipelineBundle {
            shader,
	    graphics: graphics_pipelines[0],
	    layout: pipeline_layout,
            ubo,
            pipeline_desc
        }
    }

    pub fn create_render_pass(device: &DeviceBundle, swapchain: &SwapchainBundle) -> vk::RenderPass{
        let color_attachment = vk::AttachmentDescription::default()
            .format(swapchain.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpass = [vk::SubpassDescription::default()
	    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
	    .color_attachments(&color_attachment_ref)];

        let render_pass_attachments = [color_attachment];

        let renderpass_create_info = vk::RenderPassCreateInfo::default()
	    .attachments(&render_pass_attachments)
	    .subpasses(&subpass);

        unsafe {
            device.logical.create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass!")
        }
    }

    pub fn create_framebuffers(device: &DeviceBundle, render_pass: &vk::RenderPass, image_views: &Vec<vk::ImageView>, swapchain: &SwapchainBundle) -> Vec<vk::Framebuffer>{
	let mut framebuffers = vec![];

	for &image_view in image_views.iter() {
            let attachments = [image_view];

            let framebuffer_create_info = vk::FramebufferCreateInfo::default()
                .render_pass(*render_pass)
		.attachments(&attachments)
		.width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device.logical.create_framebuffer(&framebuffer_create_info, None)
                    .expect("Failed to create Framebuffer!")
            };

            framebuffers.push(framebuffer);
	}

	framebuffers
    }

    pub fn create_command_pools(device: &DeviceBundle, num: usize, num_buffers: usize) -> Vec<CommandBundle> {
        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
	    .queue_family_index(device.queue_family_index);

        let mut commands = Vec::new();

        for _ in 0..num {

            let pool = unsafe { device.logical.create_command_pool(&command_pool_create_info, None).expect("Failed to create Command Pool!") };

            let buffers = if num_buffers > 0 {
                let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
                    .command_buffer_count(num_buffers as u32)
                    .command_pool(pool)
                    .level(vk::CommandBufferLevel::PRIMARY);

                unsafe { device.logical.allocate_command_buffers(&command_buffer_allocate_info).expect("Failed to allocate Command Buffers!") }
            } else { vec![] };

            commands.push(CommandBundle { pool, buffers})
        }

        commands
    }

    pub fn create_sync_objects(device: &DeviceBundle, num: usize) -> SyncObjectsBundle {
        let mut sync_objects = SyncObjectsBundle {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            in_flight_fences: vec![],

            spare_fences: vec![],
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();

        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..num {
            unsafe {
                let image_available_semaphore = device.logical.create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let render_finished_semaphore = device.logical.create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");

                let inflight_fence = device.logical.create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");

                let inflight_fence_2 = device.logical.create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");


                sync_objects.image_available_semaphores.push(image_available_semaphore);
                sync_objects.render_finished_semaphores.push(render_finished_semaphore);
                sync_objects.in_flight_fences.push(inflight_fence);
                sync_objects.spare_fences.push(inflight_fence_2);
            }
        }

        sync_objects
    }

    /* Create descriptor sets */
    fn create_descriptor_pool(device: &DeviceBundle, swapchain_images_size: usize) -> vk::DescriptorPool {
        let pool_sizes = [vk::DescriptorPoolSize::default().descriptor_count(swapchain_images_size as u32)];
        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::default()
            .flags(vk::DescriptorPoolCreateFlags::empty())
            .max_sets(swapchain_images_size as u32)
            .pool_sizes(&pool_sizes);

        unsafe {
            device.logical.create_descriptor_pool(&descriptor_pool_create_info, None)
                .expect("Failed to create Descriptor Pool!")
        }
    }

    fn create_descriptor_sets(
        device: &DeviceBundle,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        texture_image_view: vk::ImageView,
        texture_sampler: vk::Sampler,
        swapchain_images_size: usize,
    ) -> Vec<vk::DescriptorSet> {
        let mut layouts: Vec<vk::DescriptorSetLayout> = vec![];
        for _ in 0..swapchain_images_size {
            layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe {
            device.logical
                .allocate_descriptor_sets(&descriptor_set_allocate_info)
                .expect("Failed to allocate descriptor sets!")
        };

        for (_, &descritptor_set) in descriptor_sets.iter().enumerate() {
            let descriptor_image_infos = [
                vk::DescriptorImageInfo::default()
                    .sampler(texture_sampler)
                    .image_view(texture_image_view)
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            ];

            let descriptor_write_sets = [
                vk::WriteDescriptorSet::default()
                    .dst_set(descritptor_set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&descriptor_image_infos)
            ];

            unsafe {
                device.logical.update_descriptor_sets(&descriptor_write_sets, &[]);
            }
        }

        descriptor_sets
    }

    pub fn create_descriptor_set_layout(device: &DeviceBundle, descs: &[DescSetBinding]) -> vk::DescriptorSetLayout {

        let ubo_layout_bindings: Vec<_> = descs.iter().map(|desc| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(desc.binding)
                .descriptor_count(desc.descriptor_count)
                .descriptor_type(desc.descriptor_type)
                .stage_flags(desc.stage_flags)
        }).collect::<_>();

        let ubo_layout_create_info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::empty())
            .bindings(&ubo_layout_bindings);

        unsafe {
            device.logical.create_descriptor_set_layout(&ubo_layout_create_info, None)
                .expect("Failed to create Descriptor Set Layout!")
        }
    }

    /* Setup validation layer callbacks */
    pub fn setup_validation(entry: &ash::Entry, instance: &ash::Instance) -> (debug_utils::Instance, vk::DebugUtilsMessengerEXT) {
        let message_severity = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;

        let message_type = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION;

        let messenger_ci = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(message_severity)
            .message_type(message_type)
            .pfn_user_callback(Some(vulkan_debug_utils_callback));

        let debug_utils_loader = debug_utils::Instance::new(entry, instance);
        let utils_messenger = unsafe {
            debug_utils_loader.create_debug_utils_messenger(&messenger_ci, None).expect("Debug Utils Callback")
        };
        (debug_utils_loader, utils_messenger)
    }

}


impl Drop for VkBase {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.logical.device_wait_idle();
            self.cleanup_in_flight_buffers();

            for i in 0..self.sync_objects.image_available_semaphores.len() {
                self.device.logical.destroy_semaphore(self.sync_objects.image_available_semaphores[i], None);
                self.device.logical.destroy_semaphore(self.sync_objects.render_finished_semaphores[i], None);
                self.device.logical.destroy_fence(self.sync_objects.in_flight_fences[i], None);
            }

            for i in 0..self.sync_objects.spare_fences.len() {
                self.device.logical.destroy_fence(self.sync_objects.spare_fences[i], None);
            }

            self.cleanup_swapchain();

            for command in self.commands.iter() {
                self.device.logical.free_command_buffers(command.pool, &command.buffers);
                self.device.logical.destroy_command_pool(command.pool, None);
            }

            self.device.logical.free_command_buffers(self.spare_command.pool, &self.spare_command.buffers);
            self.device.logical.destroy_command_pool(self.spare_command.pool, None);

            self.device.logical.destroy_descriptor_pool(self.descriptor_pool, None);

            self.device.logical.destroy_device(None);
            self.surface.loader.destroy_surface(self.surface.surface, None);

            self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
}
