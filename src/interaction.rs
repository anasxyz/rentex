// src/interaction.rs

use crate::{DrawCommand, InputState, MouseButton};
use std::collections::HashSet;

/// Tracks interactive element states and handles hit testing
pub struct InteractionManager {
    /// IDs of elements currently being hovered
    hovered_elements: HashSet<usize>,
    /// IDs of elements that were hovered last frame
    prev_hovered_elements: HashSet<usize>,
}

impl InteractionManager {
    pub fn new() -> Self {
        Self {
            hovered_elements: HashSet::new(),
            prev_hovered_elements: HashSet::new(),
        }
    }

    /// Process interactions for all commands in the scene
    pub fn process_interactions(
        &mut self,
        commands: &[DrawCommand],
        input: &InputState,
    ) {
        // Update hover tracking
        self.prev_hovered_elements = self.hovered_elements.clone();
        self.hovered_elements.clear();

        let mouse_pos = input.mouse_position;

        // Check each command for interaction
        for (idx, cmd) in commands.iter().enumerate() {
            match cmd {
                DrawCommand::Button {
                    x, y, w, h, on_click, on_hover, ..
                } => {
                    let is_hovered = Self::point_in_rect(mouse_pos, (*x, *y, *w, *h));

                    if is_hovered {
                        self.hovered_elements.insert(idx);

                        // Handle click
                        if input.is_button_just_pressed(MouseButton::Left) {
                            if let Some(callback) = on_click {
                                callback();
                            }
                        }

                        // Handle hover enter
                        if !self.prev_hovered_elements.contains(&idx) {
                            if let Some(callback) = on_hover {
                                callback(true);
                            }
                        }
                    } else {
                        // Handle hover exit
                        if self.prev_hovered_elements.contains(&idx) {
                            if let Some(callback) = on_hover {
                                callback(false);
                            }
                        }
                    }
                }
                _ => {
                    // Other elements are not interactive (yet)
                }
            }
        }
    }

    /// Check if a point is inside a rectangle
    fn point_in_rect(point: (f32, f32), rect: (f32, f32, f32, f32)) -> bool {
        let (px, py) = point;
        let (x, y, w, h) = rect;
        px >= x && px <= x + w && py >= y && py <= y + h
    }

    /// Check if an element is currently hovered
    pub fn is_hovered(&self, index: usize) -> bool {
        self.hovered_elements.contains(&index)
    }

    /// Clear all interaction state
    pub fn clear(&mut self) {
        self.hovered_elements.clear();
        self.prev_hovered_elements.clear();
    }
}

impl Default for InteractionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for hit testing various shapes
pub struct HitTester;

impl HitTester {
    /// Check if point is in rectangle
    pub fn point_in_rect(point: (f32, f32), x: f32, y: f32, w: f32, h: f32) -> bool {
        let (px, py) = point;
        px >= x && px <= x + w && py >= y && py <= y + h
    }

    /// Check if point is in circle
    pub fn point_in_circle(point: (f32, f32), cx: f32, cy: f32, radius: f32) -> bool {
        let (px, py) = point;
        let dx = px - cx;
        let dy = py - cy;
        dx * dx + dy * dy <= radius * radius
    }

    /// Check if point is in rounded rectangle (approximate)
    pub fn point_in_rounded_rect(
        point: (f32, f32),
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
    ) -> bool {
        let (px, py) = point;
        let radius = radius.min(w / 2.0).min(h / 2.0);

        // Check if in main rectangles
        if Self::point_in_rect(point, x + radius, y, w - radius * 2.0, h) {
            return true;
        }
        if Self::point_in_rect(point, x, y + radius, w, h - radius * 2.0) {
            return true;
        }

        // Check corners
        let corners = [
            (x + radius, y + radius),         // Top-left
            (x + w - radius, y + radius),     // Top-right
            (x + w - radius, y + h - radius), // Bottom-right
            (x + radius, y + h - radius),     // Bottom-left
        ];

        for (cx, cy) in corners.iter() {
            if Self::point_in_circle(point, *cx, *cy, radius) {
                return true;
            }
        }

        false
    }
}
