use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use crate::style::types::Rgb;

use super::Command;

pub(crate) struct DrawRectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Rgb,
}

impl DrawRectangle {
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32, color: Rgb) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Command for DrawRectangle {
    fn execute(&self, dt: &mut DrawTarget) {
        let mut pb = PathBuilder::new();

        let source = Source::Solid(SolidSource {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: 255,
        });

        pb.rect(self.x, self.y, self.width, self.height);

        let path = pb.finish();

        dt.fill(&path, &source, &DrawOptions::new());
    }
}
