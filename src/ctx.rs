use crate::{Color, FontId, Fonts, InputState, MouseState, ShapeRenderer, TextRenderer};

pub struct Rect {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub outline_color: Color,
    pub outline_thickness: f32,
}

pub struct Text {
    pub id: u32,
    pub text: String,
    pub font_id: FontId,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
}

/// everything the user needs during setup and update
pub struct Ctx {
    pub(crate) text_renderer: TextRenderer,
    pub(crate) shape_renderer: ShapeRenderer,

    pub fonts: Fonts,
    pub mouse: MouseState,
    pub input: InputState,
    pub exit: bool,

    pub rects: Vec<Rect>,
    pub texts: Vec<Text>,

    dirty: bool,
}

impl Ctx {
    pub(crate) fn new(
        fonts: Fonts,
        text_renderer: TextRenderer,
        shape_renderer: ShapeRenderer,
    ) -> Self {
        Self {
            fonts,
            mouse: MouseState::default(),
            input: InputState::default(),
            exit: false,
            text_renderer,
            shape_renderer,
            rects: Vec::new(),
            texts: Vec::new(),
            dirty: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    
    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        outline_color: Color,
        outline_thickness: f32,
    ) {
        let new_rect = Rect {
            id: self.rects.len() as u32,
            x,
            y,
            w,
            h,
            color,
            outline_color,
            outline_thickness,
        };
        self.rects.push(new_rect);
        self.mark_dirty();
    }

    pub fn text(&mut self, text: &str, font_id: FontId, x: f32, y: f32, color: Color) {
        let entry = self.fonts.get(font_id);
        let family = entry.family.clone();
        let size = entry.size;

        let new_text = Text {
            id: self.texts.len() as u32,
            text: text.to_string(),
            font_id,
            x,
            y,
            color,
            font_size: size,
            font_family: family,
        };
        self.texts.push(new_text);
        self.mark_dirty();
    }

    pub fn circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rounded_rect(
            x,
            y,
            w,
            h,
            radius,
            color,
            outline_color,
            outline_thickness,
        );
    }
}
