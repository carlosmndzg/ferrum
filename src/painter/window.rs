use std::path::Path;

use minifb::{Window as MinifbWindow, WindowOptions};
use raqote::{DrawTarget, SolidSource};

use crate::{layout, style::types::StyledNode};

use super::{command_list::CommandList, fonts_context::FontsContext};

pub(crate) struct Window {
    window: MinifbWindow,
}

impl Window {
    pub(crate) fn new(name: &str, width: usize, height: usize) -> Self {
        let window = MinifbWindow::new(
            name,
            width,
            height,
            WindowOptions {
                resize: true,
                scale_mode: minifb::ScaleMode::UpperLeft,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self { window }
    }

    pub(crate) fn run(&mut self, root: &StyledNode, file_path: &Path) {
        let mut size = self.window.get_size();
        let mut dt = self.render(root, size, file_path);

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            let new_size = self.window.get_size();

            if new_size != (0, 0) && size != new_size {
                size = self.window.get_size();

                dt = self.render(root, size, file_path);
            }

            self.window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        }
    }

    fn render(&mut self, node: &StyledNode, size: (usize, usize), file_path: &Path) -> DrawTarget {
        let mut font_ctx = FontsContext::new();
        let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
        let root = layout::build_layout_tree(node, file_path, size);
        let commands = CommandList::new(&root, &mut font_ctx, file_path);

        self.clear_canvas(&mut dt);

        for command in &commands {
            command.execute(&mut dt, &mut font_ctx);
        }

        dt
    }

    fn clear_canvas(&mut self, dt: &mut DrawTarget) {
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));
    }
}
