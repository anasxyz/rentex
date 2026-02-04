// examples/shapes_test.rs
use std::sync::Arc;
use wgpu;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rentex::ShapeRenderer;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Shape Test")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap());

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = instance.create_surface(Arc::clone(&window)).unwrap();
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    // Create MSAA texture
    let mut msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("MSAA Texture"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 4, // 4x MSAA
        dimension: wgpu::TextureDimension::D2,
        format: surface_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let mut msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut shape_renderer = ShapeRenderer::new(&device, surface_format, config.width as f32, config.height as f32);

    let _ = event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::Resized(new_size) => {
                    config.width = new_size.width;
                    config.height = new_size.height;
                    surface.configure(&device, &config);
                    
                    // Recreate MSAA texture
                    msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("MSAA Texture"),
                        size: wgpu::Extent3d {
                            width: config.width,
                            height: config.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 4,
                        dimension: wgpu::TextureDimension::D2,
                        format: surface_format,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    });
                    msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
                    
                    shape_renderer.resize(config.width as f32, config.height as f32);
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    let frame = surface.get_current_texture().unwrap();
                    let view = frame.texture.create_view(&Default::default());
                    let mut encoder = device.create_command_encoder(&Default::default());

                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &msaa_view, // Render to MSAA texture
                                resolve_target: Some(&view), // Resolve to screen
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });

                        // Draw test shapes
                        shape_renderer.clear();
                        shape_renderer.rect(50.0, 50.0, 200.0, 100.0, [1.0, 0.0, 0.0, 1.0]); // Red rect
                        shape_renderer.circle(400.0, 150.0, 50.0, [0.0, 1.0, 0.0, 1.0]); // Green circle
                        shape_renderer.rounded_rect(50.0, 200.0, 200.0, 100.0, 20.0, [0.0, 0.0, 1.0, 1.0]); // Blue rounded rect
                        shape_renderer.render(&device, &queue, &mut pass);
                    }

                    queue.submit([encoder.finish()]);
                    frame.present();
                }
                WindowEvent::CloseRequested => {
                    target.exit();
                }
                _ => {}
            },
            _ => {}
        }
    });
}
