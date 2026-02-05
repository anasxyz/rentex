// src/scene.rs

use std::sync::Arc;

/// Callback types for interactions
pub type ClickCallback = Arc<dyn Fn() + Send + Sync>;
pub type HoverCallback = Arc<dyn Fn(bool) + Send + Sync>; // true = enter, false = exit

/// A drawing command that can be stored and replayed
#[derive(Clone)]
pub enum DrawCommand {
    Rect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        color: [f32; 4],
        outline_color: Option<[f32; 4]>,
        outline_width: f32,
    },
    Circle { 
        cx: f32, 
        cy: f32, 
        radius: f32, 
        color: [f32; 4],
        outline_color: Option<[f32; 4]>,
        outline_width: f32,
    },
    RoundedRect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        radius: f32, 
        color: [f32; 4],
        outline_color: Option<[f32; 4]>,
        outline_width: f32,
    },
    Text { 
        text: String, 
        x: f32, 
        y: f32, 
        font_size: f32,
        color: [f32; 4],
    },
    Button { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        text: String,
        fill_color: [f32; 4],
        text_color: [f32; 4],
        outline_color: Option<[f32; 4]>,
        outline_width: f32,
        hover_color: Option<[f32; 4]>,
        on_click: Option<ClickCallback>,
        on_hover: Option<HoverCallback>,
    },
}

impl std::fmt::Debug for DrawCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawCommand::Rect { x, y, w, h, color, outline_color, outline_width } => {
                f.debug_struct("Rect")
                    .field("x", x)
                    .field("y", y)
                    .field("w", w)
                    .field("h", h)
                    .field("color", color)
                    .field("outline_color", outline_color)
                    .field("outline_width", outline_width)
                    .finish()
            }
            DrawCommand::Circle { cx, cy, radius, color, outline_color, outline_width } => {
                f.debug_struct("Circle")
                    .field("cx", cx)
                    .field("cy", cy)
                    .field("radius", radius)
                    .field("color", color)
                    .field("outline_color", outline_color)
                    .field("outline_width", outline_width)
                    .finish()
            }
            DrawCommand::RoundedRect { x, y, w, h, radius, color, outline_color, outline_width } => {
                f.debug_struct("RoundedRect")
                    .field("x", x)
                    .field("y", y)
                    .field("w", w)
                    .field("h", h)
                    .field("radius", radius)
                    .field("color", color)
                    .field("outline_color", outline_color)
                    .field("outline_width", outline_width)
                    .finish()
            }
            DrawCommand::Text { text, x, y, font_size, color } => {
                f.debug_struct("Text")
                    .field("text", text)
                    .field("x", x)
                    .field("y", y)
                    .field("font_size", font_size)
                    .field("color", color)
                    .finish()
            }
            DrawCommand::Button { x, y, w, h, text, fill_color, text_color, outline_color, outline_width, hover_color, .. } => {
                f.debug_struct("Button")
                    .field("x", x)
                    .field("y", y)
                    .field("w", w)
                    .field("h", h)
                    .field("text", text)
                    .field("fill_color", fill_color)
                    .field("text_color", text_color)
                    .field("outline_color", outline_color)
                    .field("outline_width", outline_width)
                    .field("hover_color", hover_color)
                    .field("on_click", &"<callback>")
                    .field("on_hover", &"<callback>")
                    .finish()
            }
        }
    }
}

/// Builder for Rectangle
pub struct RectBuilder<'a> {
    scene: &'a mut Scene,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    outline_color: Option<[f32; 4]>,
    outline_width: f32,
}

impl<'a> RectBuilder<'a> {
    fn new(scene: &'a mut Scene, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            scene,
            x,
            y,
            w,
            h,
            color: [1.0, 1.0, 1.0, 1.0], // Default white
            outline_color: None,
            outline_width: 0.0,
        }
    }

    pub fn fill_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline_color(mut self, color: [f32; 4]) -> Self {
        self.outline_color = Some(color);
        self
    }

    pub fn outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
        self
    }
}

impl<'a> Drop for RectBuilder<'a> {
    fn drop(&mut self) {
        self.scene.commands.push(DrawCommand::Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            color: self.color,
            outline_color: self.outline_color,
            outline_width: self.outline_width,
        });
        self.scene.dirty = true;
    }
}

/// Builder for Circle
pub struct CircleBuilder<'a> {
    scene: &'a mut Scene,
    cx: f32,
    cy: f32,
    radius: f32,
    color: [f32; 4],
    outline_color: Option<[f32; 4]>,
    outline_width: f32,
}

impl<'a> CircleBuilder<'a> {
    fn new(scene: &'a mut Scene, cx: f32, cy: f32, radius: f32) -> Self {
        Self {
            scene,
            cx,
            cy,
            radius,
            color: [1.0, 1.0, 1.0, 1.0],
            outline_color: None,
            outline_width: 0.0,
        }
    }

    pub fn fill_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline_color(mut self, color: [f32; 4]) -> Self {
        self.outline_color = Some(color);
        self
    }

    pub fn outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
        self
    }
}

impl<'a> Drop for CircleBuilder<'a> {
    fn drop(&mut self) {
        self.scene.commands.push(DrawCommand::Circle {
            cx: self.cx,
            cy: self.cy,
            radius: self.radius,
            color: self.color,
            outline_color: self.outline_color,
            outline_width: self.outline_width,
        });
        self.scene.dirty = true;
    }
}

/// Builder for RoundedRect
pub struct RoundedRectBuilder<'a> {
    scene: &'a mut Scene,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: [f32; 4],
    outline_color: Option<[f32; 4]>,
    outline_width: f32,
}

impl<'a> RoundedRectBuilder<'a> {
    fn new(scene: &'a mut Scene, x: f32, y: f32, w: f32, h: f32, radius: f32) -> Self {
        Self {
            scene,
            x,
            y,
            w,
            h,
            radius,
            color: [1.0, 1.0, 1.0, 1.0],
            outline_color: None,
            outline_width: 0.0,
        }
    }

    pub fn fill_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline_color(mut self, color: [f32; 4]) -> Self {
        self.outline_color = Some(color);
        self
    }

    pub fn outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
        self
    }
}

impl<'a> Drop for RoundedRectBuilder<'a> {
    fn drop(&mut self) {
        self.scene.commands.push(DrawCommand::RoundedRect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            radius: self.radius,
            color: self.color,
            outline_color: self.outline_color,
            outline_width: self.outline_width,
        });
        self.scene.dirty = true;
    }
}

/// Builder for Text
pub struct TextBuilder<'a> {
    scene: &'a mut Scene,
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    color: [f32; 4],
}

impl<'a> TextBuilder<'a> {
    fn new(scene: &'a mut Scene, text: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            scene,
            text: text.into(),
            x,
            y,
            font_size: 22.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

impl<'a> Drop for TextBuilder<'a> {
    fn drop(&mut self) {
        self.scene.commands.push(DrawCommand::Text {
            text: self.text.clone(),
            x: self.x,
            y: self.y,
            font_size: self.font_size,
            color: self.color,
        });
        self.scene.dirty = true;
    }
}

/// Builder for Button
pub struct ButtonBuilder<'a> {
    scene: &'a mut Scene,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text: String,
    fill_color: [f32; 4],
    text_color: [f32; 4],
    outline_color: Option<[f32; 4]>,
    outline_width: f32,
    hover_color: Option<[f32; 4]>,
    on_click: Option<ClickCallback>,
    on_hover: Option<HoverCallback>,
}

impl<'a> ButtonBuilder<'a> {
    fn new(scene: &'a mut Scene, x: f32, y: f32, w: f32, h: f32, text: impl Into<String>) -> Self {
        Self {
            scene,
            x,
            y,
            w,
            h,
            text: text.into(),
            fill_color: [0.2, 0.4, 0.8, 1.0], // Default blue
            text_color: [1.0, 1.0, 1.0, 1.0], // Default white
            outline_color: None,
            outline_width: 0.0,
            hover_color: None,
            on_click: None,
            on_hover: None,
        }
    }

    pub fn fill_color(mut self, color: [f32; 4]) -> Self {
        self.fill_color = color;
        self
    }

    pub fn text_color(mut self, color: [f32; 4]) -> Self {
        self.text_color = color;
        self
    }

    pub fn outline_color(mut self, color: [f32; 4]) -> Self {
        self.outline_color = Some(color);
        self
    }

    pub fn outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
        self
    }

    pub fn hover_color(mut self, color: [f32; 4]) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
        self
    }

    pub fn on_hover<F>(mut self, callback: F) -> Self 
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.on_hover = Some(Arc::new(callback));
        self
    }
}

impl<'a> Drop for ButtonBuilder<'a> {
    fn drop(&mut self) {
        self.scene.commands.push(DrawCommand::Button {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            text: self.text.clone(),
            fill_color: self.fill_color,
            text_color: self.text_color,
            outline_color: self.outline_color,
            outline_width: self.outline_width,
            hover_color: self.hover_color,
            on_click: self.on_click.clone(),
            on_hover: self.on_hover.clone(),
        });
        self.scene.dirty = true;
    }
}

/// Simple scene graph - just a list of drawing commands
pub struct Scene {
    commands: Vec<DrawCommand>,
    dirty: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            dirty: true,
        }
    }

    /// Add a rectangle to the scene (returns builder)
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) -> RectBuilder {
        RectBuilder::new(self, x, y, w, h)
    }

    /// Add a circle to the scene (returns builder)
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32) -> CircleBuilder {
        CircleBuilder::new(self, cx, cy, radius)
    }

    /// Add a rounded rectangle to the scene (returns builder)
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32) -> RoundedRectBuilder {
        RoundedRectBuilder::new(self, x, y, w, h, radius)
    }

    /// Add text to the scene (returns builder)
    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) -> TextBuilder {
        TextBuilder::new(self, text, x, y)
    }

    /// Add a button to the scene (returns builder)
    pub fn button(&mut self, x: f32, y: f32, w: f32, h: f32, text: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(self, x, y, w, h, text)
    }

    /// Clear all commands
    pub fn clear(&mut self) {
        self.commands.clear();
        self.dirty = true;
    }

    /// Get all commands (for rendering)
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Check if scene needs re-rendering
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark scene as clean (called after rendering)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Force a re-render on next frame
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}
