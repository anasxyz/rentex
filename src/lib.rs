// src/lib.rs

mod shapes;
mod text;
mod app;
mod scene;
mod widgets;
mod input;
mod interaction;

pub use shapes::ShapeRenderer;
pub use text::TextRenderer;
pub use app::{App, Rntx};
pub use scene::{Scene, DrawCommand};
pub use widgets::WidgetRenderer;
pub use input::{InputState, MouseButton, MouseButtonEvent};
pub use interaction::{InteractionManager, HitTester};
