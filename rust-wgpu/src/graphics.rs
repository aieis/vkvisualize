use winit::{event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoopWindowTarget, keyboard::{KeyCode, PhysicalKey}, window::Window};
use wgpu::util::DeviceExt;
use pollster::FutureExt as _;
use bytemuck::{Pod, Zeroable};


use crate::{k4a, texture::Texture};

const OUT_WIDTH : usize = 60;
const OUT_HEIGHT : usize = 40;
const D : [char; 11] = [' ', '.', ',', '*', '+', ':', ';', '#', '@', '%', ' '];

pub struct State<'a> {
    app: crate::app::App,

    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture: Texture,
    diffuse_bind_group: wgpu::BindGroup,
    window: &'a Window,
}

impl<'a> State<'a> {
    pub fn new(window: &'a Window, app: crate::app::App) -> State <'a>{
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).block_on().unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).block_on().unwrap();


        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX
        });

        let num_indices = INDICES.len() as u32;

        let im = app.depth[0].as_ref().unwrap();
        let texture = Texture::from_bytes(
            &device, &queue, wgpu::TextureFormat::Depth16Unorm, "Texture",
            im.get_buffer(), im.width as u32, im.height as u32, im.width as u32 * 2
        );
        
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false, sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                }
            ],
            label: Some("texture_bind_group_layout")
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&texture.sampler) }
            ],
            label: Some("diffuse_bind_group")
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("../assets/shaders/shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main",
                                        buffers: &[Vertex::desc()], compilation_options: wgpu::PipelineCompilationOptions::default() },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL, })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            multiview: None,
            cache: None,
        });


        State {
            app,
            window,
            surface,
            device,
            queue,
            config,
            size,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture,
            diffuse_bind_group,
            render_pipeline
        }
    }

    pub fn update(&mut self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        if should_close(event) {
            self.stop();
            control_flow.exit();
        }

        self.app.update();

        if let Some(im) = self.app.depth[0].as_ref() {
            self.texture.update(&self.queue, &im.get_buffer());
        }
        
        match event {
            WindowEvent::Resized(size) => { self.resize(*size); }
            WindowEvent::RedrawRequested => {
                self.window().request_redraw();
                let _ = self.draw();
            }
            _ => {}
        }
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: Some("Render Encoder")});

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0, }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn stop(&self) {
        self.app.stop();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn window(&self) -> &Window {
        self.window
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        }
    }
}
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], },
    Vertex { position: [ 1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [ 1.0,  1.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [-1.0,  1.0, 0.0], tex_coords: [1.0, 0.0], }
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    3, 0, 2
];


#[allow(dead_code)]
fn print_depth(depth: k4a::Image) {
    if depth.width == 0 {
        return;
    }

    let depth_buf = depth.get_buffer();

    let stride = depth.width as usize * 2;
    for h in 0..OUT_HEIGHT {
        let py = ((depth.height as f64 / OUT_HEIGHT as f64) * h as f64) as usize;
        for c in 0..OUT_WIDTH {
            let px = ((depth.width as f64 / OUT_WIDTH as f64) * c as f64) as usize;
            let r = depth_buf[py * stride + px * 2];
            let v = ((r as f32 / 255.0) * 9.0) as usize;
            print!("{}", D[10 - v]);
        }
        println!();
    }
}


fn should_close(event: &WindowEvent) -> bool {
    match event {
        WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            }
        | WindowEvent::KeyboardInput {
            event:
            KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::KeyQ),
                ..
            },
            ..
        }
        => {true},
        _ => {false}
    }
}
