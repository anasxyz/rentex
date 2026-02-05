// src/input.rs

/// Mouse button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Mouse button event
#[derive(Debug, Clone, Copy)]
pub enum MouseButtonEvent {
    Pressed(MouseButton),
    Released(MouseButton),
}

/// Input state tracking
pub struct InputState {
    /// Current mouse position in logical coordinates
    pub mouse_position: (f32, f32),
    /// Previous mouse position (for delta calculation)
    pub mouse_position_prev: (f32, f32),
    /// Currently pressed mouse buttons
    pressed_buttons: [bool; 3], // Left, Right, Middle
    /// Mouse buttons pressed this frame
    just_pressed: [bool; 3],
    /// Mouse buttons released this frame
    just_released: [bool; 3],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_position_prev: (0.0, 0.0),
            pressed_buttons: [false; 3],
            just_pressed: [false; 3],
            just_released: [false; 3],
        }
    }

    /// Clear per-frame state (call at start of each frame)
    pub fn begin_frame(&mut self) {
        self.just_pressed = [false; 3];
        self.just_released = [false; 3];
        self.mouse_position_prev = self.mouse_position;
    }

    /// Update mouse position
    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }

    /// Handle mouse button press
    pub fn press_mouse_button(&mut self, button: MouseButton) {
        let idx = button as usize;
        if !self.pressed_buttons[idx] {
            self.pressed_buttons[idx] = true;
            self.just_pressed[idx] = true;
        }
    }

    /// Handle mouse button release
    pub fn release_mouse_button(&mut self, button: MouseButton) {
        let idx = button as usize;
        if self.pressed_buttons[idx] {
            self.pressed_buttons[idx] = false;
            self.just_released[idx] = true;
        }
    }

    /// Check if button is currently pressed
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons[button as usize]
    }

    /// Check if button was just pressed this frame
    pub fn is_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed[button as usize]
    }

    /// Check if button was just released this frame
    pub fn is_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released[button as usize]
    }

    /// Get mouse delta since last frame
    pub fn mouse_delta(&self) -> (f32, f32) {
        (
            self.mouse_position.0 - self.mouse_position_prev.0,
            self.mouse_position.1 - self.mouse_position_prev.1,
        )
    }

    /// Check if mouse position changed this frame
    pub fn mouse_moved(&self) -> bool {
        let delta = self.mouse_delta();
        delta.0.abs() > 0.001 || delta.1.abs() > 0.001
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
