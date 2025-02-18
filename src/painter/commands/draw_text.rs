use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

use crate::{painter::fonts_context::FontsContext, style::types::Rgb};

use super::Command;

pub(crate) struct DrawText {
    x: f32,
    y: f32,
    text: String,
    font_size: f32,
    font_weight: u32,
    color: Rgb,
}

impl DrawText {
    pub(crate) fn new(
        x: f32,
        y: f32,
        text: String,
        font_size: f32,
        font_weight: u32,
        color: Rgb,
    ) -> Self {
        Self {
            x,
            y,
            text,
            font_size,
            font_weight,
            color,
        }
    }
}

impl Command for DrawText {
    fn execute(&self, dt: &mut DrawTarget, font_ctx: &mut FontsContext) {
        let source = Source::Solid(SolidSource {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: 255,
        });

        let font = font_ctx.get_font(self.font_weight);

        // 0.98 is a micro adjustment because raqote draws text a bit smaller.

        dt.draw_text(
            font,
            self.font_size,
            &self.text,
            Point::new(self.x, self.y),
            &source,
            &DrawOptions::new(),
        );
    }
}
