// src/shapes.rs

use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    screen_width: f32,
    screen_height: f32,
}

impl ShapeRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shape.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &vertex_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Vertex Buffer"),
            size: 1024 * std::mem::size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertices: Vec::new(),
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    fn to_ndc(&self, x: f32, y: f32) -> [f32; 2] {
        [
            (x / self.screen_width) * 2.0 - 1.0,
            1.0 - (y / self.screen_height) * 2.0,
        ]
    }

    /// Draw a filled rectangle with outline
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        // Draw fill
        let p1 = self.to_ndc(x, y);
        let p2 = self.to_ndc(x + w, y);
        let p3 = self.to_ndc(x, y + h);
        let p4 = self.to_ndc(x + w, y + h);
        
        self.vertices.extend_from_slice(&[
            Vertex { position: p1, color },
            Vertex { position: p2, color },
            Vertex { position: p3, color },
            Vertex { position: p2, color },
            Vertex { position: p4, color },
            Vertex { position: p3, color },
        ]);

        // Draw outline if thickness > 0
        if outline_thickness > 0.0 {
            let half = outline_thickness / 2.0;
            self.rect_outline(x, y, w, h, outline_color, half);
        }
    }

    fn rect_outline(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], half: f32) {
        // Top
        let p1 = self.to_ndc(x - half, y - half);
        let p2 = self.to_ndc(x + w + half, y - half);
        let p3 = self.to_ndc(x - half, y + half);
        let p4 = self.to_ndc(x + w + half, y + half);
        self.vertices.extend_from_slice(&[
            Vertex { position: p1, color }, Vertex { position: p2, color }, Vertex { position: p3, color },
            Vertex { position: p2, color }, Vertex { position: p4, color }, Vertex { position: p3, color },
        ]);

        // Bottom
        let p1 = self.to_ndc(x - half, y + h - half);
        let p2 = self.to_ndc(x + w + half, y + h - half);
        let p3 = self.to_ndc(x - half, y + h + half);
        let p4 = self.to_ndc(x + w + half, y + h + half);
        self.vertices.extend_from_slice(&[
            Vertex { position: p1, color }, Vertex { position: p2, color }, Vertex { position: p3, color },
            Vertex { position: p2, color }, Vertex { position: p4, color }, Vertex { position: p3, color },
        ]);

        // Left
        let p1 = self.to_ndc(x - half, y + half);
        let p2 = self.to_ndc(x + half, y + half);
        let p3 = self.to_ndc(x - half, y + h - half);
        let p4 = self.to_ndc(x + half, y + h - half);
        self.vertices.extend_from_slice(&[
            Vertex { position: p1, color }, Vertex { position: p2, color }, Vertex { position: p3, color },
            Vertex { position: p2, color }, Vertex { position: p4, color }, Vertex { position: p3, color },
        ]);

        // Right
        let p1 = self.to_ndc(x + w - half, y + half);
        let p2 = self.to_ndc(x + w + half, y + half);
        let p3 = self.to_ndc(x + w - half, y + h - half);
        let p4 = self.to_ndc(x + w + half, y + h - half);
        self.vertices.extend_from_slice(&[
            Vertex { position: p1, color }, Vertex { position: p2, color }, Vertex { position: p3, color },
            Vertex { position: p2, color }, Vertex { position: p4, color }, Vertex { position: p3, color },
        ]);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    /// Draw a filled circle with outline
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        let segments = 32;
        let pi = std::f32::consts::PI;
        
        // Draw fill
        let center = self.to_ndc(cx, cy);
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * 2.0 * pi;
            let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * pi;
            
            let p1 = self.to_ndc(cx + radius * angle1.cos(), cy + radius * angle1.sin());
            let p2 = self.to_ndc(cx + radius * angle2.cos(), cy + radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: center, color },
                Vertex { position: p1, color },
                Vertex { position: p2, color },
            ]);
        }

        // Draw outline if thickness > 0
        if outline_thickness > 0.0 {
            self.circle_outline(cx, cy, radius, outline_color, outline_thickness);
        }
    }

    fn circle_outline(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], thickness: f32) {
        let segments = 32;
        let pi = std::f32::consts::PI;
        let inner_radius = radius - thickness / 2.0;
        let outer_radius = radius + thickness / 2.0;
        
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * 2.0 * pi;
            let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * pi;
            
            let inner1 = self.to_ndc(cx + inner_radius * angle1.cos(), cy + inner_radius * angle1.sin());
            let inner2 = self.to_ndc(cx + inner_radius * angle2.cos(), cy + inner_radius * angle2.sin());
            let outer1 = self.to_ndc(cx + outer_radius * angle1.cos(), cy + outer_radius * angle1.sin());
            let outer2 = self.to_ndc(cx + outer_radius * angle2.cos(), cy + outer_radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: inner1, color },
                Vertex { position: outer1, color },
                Vertex { position: inner2, color },
                Vertex { position: outer1, color },
                Vertex { position: outer2, color },
                Vertex { position: inner2, color },
            ]);
        }
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    /// Draw a rounded rectangle with outline
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        let radius = radius.min(w / 2.0).min(h / 2.0);
        
        // Draw fill
        self.rect(x + radius, y, w - radius * 2.0, h, color, [0.0; 4], 0.0);
        self.rect(x, y + radius, radius, h - radius * 2.0, color, [0.0; 4], 0.0);
        self.rect(x + w - radius, y + radius, radius, h - radius * 2.0, color, [0.0; 4], 0.0);
        
        self.quarter_circle(x + radius, y + radius, radius, color, 2);
        self.quarter_circle(x + w - radius, y + radius, radius, color, 3);
        self.quarter_circle(x + w - radius, y + h - radius, radius, color, 0);
        self.quarter_circle(x + radius, y + h - radius, radius, color, 1);

        // Draw outline if thickness > 0
        if outline_thickness > 0.0 {
            self.rounded_rect_outline(x, y, w, h, radius, outline_color, outline_thickness);
        }
    }

    fn rounded_rect_outline(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], thickness: f32) {
        let half = thickness / 2.0;
        
        // Edges
        self.rect(x + radius, y - half, w - radius * 2.0, thickness, color, [0.0; 4], 0.0);
        self.rect(x + radius, y + h - half, w - radius * 2.0, thickness, color, [0.0; 4], 0.0);
        self.rect(x - half, y + radius, thickness, h - radius * 2.0, color, [0.0; 4], 0.0);
        self.rect(x + w - half, y + radius, thickness, h - radius * 2.0, color, [0.0; 4], 0.0);
        
        // Corner outlines
        self.quarter_circle_outline(x + radius, y + radius, radius, color, thickness, 2);
        self.quarter_circle_outline(x + w - radius, y + radius, radius, color, thickness, 3);
        self.quarter_circle_outline(x + w - radius, y + h - radius, radius, color, thickness, 0);
        self.quarter_circle_outline(x + radius, y + h - radius, radius, color, thickness, 1);
    }

    pub fn draw_rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.rounded_rect(x, y, w, h, radius, color, outline_color, outline_thickness);
    }

    fn quarter_circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], quarter: u32) {
        let segments = 8;
        let pi = std::f32::consts::PI;
        let start_angle = quarter as f32 * pi / 2.0;
        let center = self.to_ndc(cx, cy);
        
        for i in 0..segments {
            let angle1 = start_angle + (i as f32 / segments as f32) * pi / 2.0;
            let angle2 = start_angle + ((i + 1) as f32 / segments as f32) * pi / 2.0;
            
            let p1 = self.to_ndc(cx + radius * angle1.cos(), cy + radius * angle1.sin());
            let p2 = self.to_ndc(cx + radius * angle2.cos(), cy + radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: center, color },
                Vertex { position: p1, color },
                Vertex { position: p2, color },
            ]);
        }
    }

    fn quarter_circle_outline(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], thickness: f32, quarter: u32) {
        let segments = 8;
        let pi = std::f32::consts::PI;
        let start_angle = quarter as f32 * pi / 2.0;
        let inner_radius = radius - thickness / 2.0;
        let outer_radius = radius + thickness / 2.0;
        
        for i in 0..segments {
            let angle1 = start_angle + (i as f32 / segments as f32) * pi / 2.0;
            let angle2 = start_angle + ((i + 1) as f32 / segments as f32) * pi / 2.0;
            
            let inner1 = self.to_ndc(cx + inner_radius * angle1.cos(), cy + inner_radius * angle1.sin());
            let inner2 = self.to_ndc(cx + inner_radius * angle2.cos(), cy + inner_radius * angle2.sin());
            let outer1 = self.to_ndc(cx + outer_radius * angle1.cos(), cy + outer_radius * angle1.sin());
            let outer2 = self.to_ndc(cx + outer_radius * angle2.cos(), cy + outer_radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: inner1, color },
                Vertex { position: outer1, color },
                Vertex { position: inner2, color },
                Vertex { position: outer1, color },
                Vertex { position: outer2, color },
                Vertex { position: inner2, color },
            ]);
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.vertices.is_empty() {
            return;
        }

        let vertex_data = bytemuck::cast_slice(&self.vertices);
        let required_size = vertex_data.len() as u64;
        
        if required_size > self.vertex_buffer.size() {
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shape Vertex Buffer"),
                size: required_size * 2,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        
        queue.write_buffer(&self.vertex_buffer, 0, vertex_data);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertices.len() as u32, 0..1);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}
