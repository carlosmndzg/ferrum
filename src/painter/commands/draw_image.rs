use std::path::PathBuf;

use image::{imageops::FilterType::Nearest, GenericImageView};
use raqote::{DrawOptions, DrawTarget, Image};

use crate::painter::fonts_context::FontsContext;

use super::Command;

pub(crate) struct DrawImage {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    path: PathBuf,
}

impl DrawImage {
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32, path: PathBuf) -> Self {
        Self {
            x,
            y,
            width,
            height,
            path,
        }
    }
}

impl Command for DrawImage {
    fn execute(&self, dt: &mut DrawTarget, _font_ctx: &mut FontsContext) {
        let Ok(img) = image::open(&self.path) else {
            return;
        };

        let img = img.resize_exact(self.width as u32, self.height as u32, Nearest);
        let (img_width, img_height) = img.dimensions();
        let mut argb_data = vec![0; (img_width * img_height) as usize];

        for (x, y, pixel) in img.to_rgba8().enumerate_pixels() {
            let idx = (y * img_width + x) as usize;
            let [r, g, b, a] = pixel.0;
            argb_data[idx] = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        }

        let image = Image {
            width: img_width as i32,
            height: img_height as i32,
            data: &argb_data,
        };

        dt.draw_image_at(self.x, self.y, &image, &DrawOptions::default());
    }
}
