use font_kit::{
    family_name::FamilyName,
    properties::{Properties, Weight},
    source::SystemSource,
};
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

use crate::style::types::Rgb;

use super::Command;

pub(crate) struct DrawText {
    x: f32,
    y: f32,
    text: String,
    font_size: f32,
    font_weight: f32,
    color: Rgb,
}

impl DrawText {
    pub(crate) fn new(
        x: f32,
        y: f32,
        text: String,
        font_size: f32,
        font_weight: f32,
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
    fn execute(&self, dt: &mut DrawTarget) {
        let font = SystemSource::new()
            .select_best_match(
                &[FamilyName::SansSerif],
                &Properties {
                    weight: Weight(self.font_weight),
                    ..Default::default()
                },
            )
            .unwrap()
            .load()
            .unwrap();

        let source = Source::Solid(SolidSource {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: 255,
        });

        // 0.98 is a micro adjustment because raqote draws text a bit smaller.

        dt.draw_text(
            &font,
            self.font_size,
            &self.text,
            Point::new(self.x, self.y + self.font_size * 0.94),
            &source,
            &DrawOptions::new(),
        );
    }
}
