// src/scene.rs - OPTIMIZED

/// A drawing command that can be stored and replayed
#[derive(Clone, Debug)]
pub enum DrawCommand {
    Rect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    },
    Circle { 
        cx: f32, 
        cy: f32, 
        radius: f32, 
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    },
    RoundedRect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        radius: f32, 
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    },
    Text { 
        text: String, 
        x: f32, 
        y: f32 
    },
}

/// Simple scene graph - optimized for retained mode rendering
pub struct Scene {
    commands: Vec<DrawCommand>,
    dirty: bool,
    // Optimization: Track capacity to avoid reallocations
    initial_capacity: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self::with_capacity(64) // Reasonable default
    }

    /// Create scene with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
            dirty: true,
            initial_capacity: capacity,
        }
    }

    /// Add a rectangle to the scene
    #[inline]
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.commands.push(DrawCommand::Rect { x, y, w, h, color, outline_color, outline_thickness });
        self.dirty = true;
    }

    /// Add a circle to the scene
    #[inline]
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.commands.push(DrawCommand::Circle { cx, cy, radius, color, outline_color, outline_thickness });
        self.dirty = true;
    }

    /// Add a rounded rectangle to the scene
    #[inline]
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.commands.push(DrawCommand::RoundedRect { x, y, w, h, radius, color, outline_color, outline_thickness });
        self.dirty = true;
    }

    /// Add text to the scene
    #[inline]
    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) {
        self.commands.push(DrawCommand::Text { 
            text: text.into(), 
            x, 
            y 
        });
        self.dirty = true;
    }

    /// Clear all commands
    #[inline]
    pub fn clear(&mut self) {
        // Optimization: Don't deallocate, just reset length
        self.commands.clear();
        self.dirty = true;
        
        // Optimization: Shrink if we're using way more than initial capacity
        if self.commands.capacity() > self.initial_capacity * 4 {
            self.commands.shrink_to(self.initial_capacity * 2);
        }
    }

    /// Get all commands (for rendering)
    #[inline(always)]
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Check if scene needs re-rendering
    #[inline(always)]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark scene as clean (called after rendering)
    #[inline(always)]
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Force a re-render on next frame
    #[inline(always)]
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Get number of commands in scene
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if scene is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
