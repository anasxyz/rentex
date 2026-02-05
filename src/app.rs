// src/app.rs

use std::sync::Arc;
use wgpu;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton as WinitMouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{ShapeRenderer, TextRenderer, Scene, WidgetRenderer, InputState, InteractionManager, MouseButton};

pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    surface_format: wgpu::TextureFormat,
    msaa_texture: wgpu::Texture,
    msaa_view: wgpu::TextureView,
    scale_factor: f64,
}

pub struct Rntx<'a> {
    pub scene: &'a mut Scene,
    pub shapes: &'a mut ShapeRenderer,
    pub text: &'a mut TextRenderer,
    pub widgets: &'a mut WidgetRenderer,
    pub input: &'a InputState,
    pub width: f32,
    pub height: f32,
    pub scale_factor: f64,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        pollster::block_on(Self::new_async(title, width, height))
    }

    async fn new_async(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                .build(&event_loop)
                .unwrap(),
        );

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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let physical_size = window.inner_size();
        let scale_factor = window.scale_factor();
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let msaa_texture = Self::create_msaa_texture(&device, &config, surface_format);
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            event_loop: Some(event_loop),
            window,
            device,
            queue,
            surface,
            config,
            surface_format,
            msaa_texture,
            msaa_view,
            scale_factor,
        }
    }

    fn create_msaa_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        format: wgpu::TextureFormat,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }

    pub fn run<F>(mut self, mut update_fn: F)
    where
        F: FnMut(&mut Rntx) + 'static,
    {
        let mut shape_renderer = ShapeRenderer::new(
            &self.device,
            self.surface_format,
            (self.config.width as f64 / self.scale_factor) as f32,
            (self.config.height as f64 / self.scale_factor) as f32,
        );

        let mut text_renderer = TextRenderer::new(&self.device, &self.queue, self.surface_format);
        text_renderer.resize(
            (self.config.width as f64 / self.scale_factor) as f32,
            (self.config.height as f64 / self.scale_factor) as f32,
            self.scale_factor,
        );
        
        let mut widget_renderer = WidgetRenderer::new();
        let mut scene = Scene::new();
        let mut input_state = InputState::new();
        let mut interaction_manager = InteractionManager::new();

        // Helper to check if mouse is hovering over any button
        let check_hover = |commands: &[crate::DrawCommand], pos: (f32, f32)| -> bool {
            let (px, py) = pos;
            for cmd in commands {
                if let crate::DrawCommand::Button { x, y, w, h, .. } = cmd {
                    if px >= *x && px <= x + w && py >= *y && py <= y + h {
                        return true;
                    }
                }
            }
            false
        };

        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        // Convert physical position to logical coordinates
                        let logical_x = position.x as f64 / self.scale_factor;
                        let logical_y = position.y as f64 / self.scale_factor;
                        
                        let old_pos = input_state.mouse_position;
                        input_state.update_mouse_position(logical_x as f32, logical_y as f32);
                        let new_pos = input_state.mouse_position;
                        
                        // Only redraw if hover state changed (entered or exited a button)
                        let old_hovered = check_hover(scene.commands(), old_pos);
                        let new_hovered = check_hover(scene.commands(), new_pos);
                        
                        if old_hovered != new_hovered {
                            self.window.request_redraw();
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        let mouse_button = match button {
                            WinitMouseButton::Left => MouseButton::Left,
                            WinitMouseButton::Right => MouseButton::Right,
                            WinitMouseButton::Middle => MouseButton::Middle,
                            _ => return,
                        };

                        match state {
                            ElementState::Pressed => {
                                input_state.press_mouse_button(mouse_button);
                            }
                            ElementState::Released => {
                                input_state.release_mouse_button(mouse_button);
                            }
                        }
                        
                        self.window.request_redraw();
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        self.scale_factor = scale_factor;
                        let physical_size = self.window.inner_size();
                        self.config.width = physical_size.width;
                        self.config.height = physical_size.height;
                        self.surface.configure(&self.device, &self.config);

                        self.msaa_texture = Self::create_msaa_texture(
                            &self.device,
                            &self.config,
                            self.surface_format,
                        );
                        self.msaa_view = self
                            .msaa_texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let logical_width = (self.config.width as f64 / self.scale_factor) as f32;
                        let logical_height = (self.config.height as f64 / self.scale_factor) as f32;
                        
                        shape_renderer.resize(logical_width, logical_height);
                        text_renderer.resize(logical_width, logical_height, self.scale_factor);
                        scene.mark_dirty();
                        self.window.request_redraw();
                    }
                    WindowEvent::Resized(new_size) => {
                        self.config.width = new_size.width;
                        self.config.height = new_size.height;
                        self.surface.configure(&self.device, &self.config);

                        self.msaa_texture = Self::create_msaa_texture(
                            &self.device,
                            &self.config,
                            self.surface_format,
                        );
                        self.msaa_view = self
                            .msaa_texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let logical_width = (self.config.width as f64 / self.scale_factor) as f32;
                        let logical_height = (self.config.height as f64 / self.scale_factor) as f32;
                        
                        shape_renderer.resize(logical_width, logical_height);
                        text_renderer.resize(logical_width, logical_height, self.scale_factor);
                        scene.mark_dirty();
                        self.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        // Only rebuild scene if dirty (retained mode)
                        if scene.is_dirty() {
                            scene.clear();
                            let mut rntx = Rntx {
                                scene: &mut scene,
                                shapes: &mut shape_renderer,
                                text: &mut text_renderer,
                                widgets: &mut widget_renderer,
                                input: &input_state,
                                width: (self.config.width as f64 / self.scale_factor) as f32,
                                height: (self.config.height as f64 / self.scale_factor) as f32,
                                scale_factor: self.scale_factor,
                            };
                            update_fn(&mut rntx);
                        }

                        // Always process interactions (even if scene not dirty)
                        interaction_manager.process_interactions(scene.commands(), &input_state);

                        // Render the scene
                        let frame = self.surface.get_current_texture().unwrap();
                        let view = frame.texture.create_view(&Default::default());
                        let mut encoder = self
                            .device
                            .create_command_encoder(&Default::default());

                        {
                            let mut pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(
                                        wgpu::RenderPassColorAttachment {
                                            view: &self.msaa_view,
                                            resolve_target: Some(&view),
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                                store: wgpu::StoreOp::Store,
                                            },
                                        },
                                    )],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            // Clear and rebuild vertices from scene
                            shape_renderer.clear();
                            text_renderer.clear();
                            
                            // Process all commands
                            for (idx, cmd) in scene.commands().iter().enumerate() {
                                match cmd {
                                    crate::DrawCommand::Rect { x, y, w, h, color, outline_color, outline_width } => {
                                        shape_renderer.rect(*x, *y, *w, *h, *color);
                                        if let Some(outline) = outline_color {
                                            if *outline_width > 0.0 {
                                                shape_renderer.rect_outline(*x, *y, *w, *h, *outline_width, *outline);
                                            }
                                        }
                                    }
                                    crate::DrawCommand::Circle { cx, cy, radius, color, outline_color, outline_width } => {
                                        shape_renderer.circle(*cx, *cy, *radius, *color);
                                        if let Some(outline) = outline_color {
                                            if *outline_width > 0.0 {
                                                shape_renderer.circle_outline(*cx, *cy, *radius, *outline_width, *outline);
                                            }
                                        }
                                    }
                                    crate::DrawCommand::RoundedRect { x, y, w, h, radius, color, outline_color, outline_width } => {
                                        shape_renderer.rounded_rect(*x, *y, *w, *h, *radius, *color);
                                        if let Some(outline) = outline_color {
                                            if *outline_width > 0.0 {
                                                shape_renderer.rounded_rect_outline(*x, *y, *w, *h, *radius, *outline_width, *outline);
                                            }
                                        }
                                    }
                                    crate::DrawCommand::Text { text, x, y, font_size, color } => {
                                        text_renderer.queue_text(
                                            text,
                                            *x,
                                            *y,
                                            (self.config.width as f64 / self.scale_factor) as f32,
                                            (self.config.height as f64 / self.scale_factor) as f32,
                                            self.scale_factor,
                                            *font_size,
                                            *color,
                                        );
                                    }
                                    crate::DrawCommand::Button { 
                                        x, y, w, h, text, fill_color, text_color, 
                                        outline_color, outline_width, hover_color, ..
                                    } => {
                                        // Use hover color if this button is being hovered
                                        let current_color = if interaction_manager.is_hovered(idx) {
                                            hover_color.unwrap_or(*fill_color)
                                        } else {
                                            *fill_color
                                        };

                                        // Draw button background
                                        shape_renderer.rounded_rect(*x, *y, *w, *h, 8.0, current_color);
                                        
                                        // Draw outline if specified
                                        if let Some(outline) = outline_color {
                                            if *outline_width > 0.0 {
                                                shape_renderer.rounded_rect_outline(*x, *y, *w, *h, 8.0, *outline_width, *outline);
                                            }
                                        }
                                        
                                        // Measure and center text
                                        let base_font_size = 22.0;
                                        let available_width = w - 5.0;
                                        let available_height = h - 10.0;
                                        
                                        let (text_width, _) = text_renderer.measure_text(text, base_font_size);
                                        
                                        let scale_w = if text_width > available_width { 
                                            available_width / text_width 
                                        } else { 
                                            1.0 
                                        };
                                        let scale_h = if base_font_size > available_height { 
                                            available_height / base_font_size 
                                        } else { 
                                            1.0 
                                        };
                                        let font_size = base_font_size * scale_w.min(scale_h);
                                        
                                        let (final_w, _) = text_renderer.measure_text(text, font_size);
                                        
                                        let text_x = x + (w - final_w) / 2.0;
                                        let text_y = y + h / 2.0 - font_size * 0.69;
                                        
                                        text_renderer.queue_text(
                                            text,
                                            text_x,
                                            text_y,
                                            (self.config.width as f64 / self.scale_factor) as f32,
                                            (self.config.height as f64 / self.scale_factor) as f32,
                                            self.scale_factor,
                                            font_size,
                                            *text_color,
                                        );
                                    }
                                }
                            }

                            // Render all shapes
                            shape_renderer.render(&self.device, &self.queue, &mut pass);

                            // Render all text
                            text_renderer.render(
                                (self.config.width as f64 / self.scale_factor) as f32,
                                (self.config.height as f64 / self.scale_factor) as f32,
                                self.scale_factor,
                                &self.device,
                                &self.queue,
                                &mut pass,
                            );
                        }

                        self.queue.submit([encoder.finish()]);
                        frame.present();
                        
                        scene.mark_clean();
                        
                        // Clear per-frame input state after processing
                        input_state.begin_frame();
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
}
