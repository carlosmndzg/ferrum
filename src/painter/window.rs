use minifb::Window as MinifbWindow;
use raqote::{DrawTarget, SolidSource};

use crate::layout::types::LayoutNode;

use super::command_list::CommandList;

pub(crate) struct Window {
    window: MinifbWindow,
}

impl Window {
    pub(crate) fn new(name: &str, width: usize, height: usize) -> Self {
        let window =
            MinifbWindow::new(name, width, height, minifb::WindowOptions::default()).unwrap();

        Self { window }
    }

    pub(crate) fn run(&mut self, root: &LayoutNode) {
        let commands = CommandList::new(root);

        let size = self.window.get_size();
        let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

        self.clear_canvas(&mut dt);

        for command in &commands.commands {
            command.execute(&mut dt);
        }

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        }
    }

    fn clear_canvas(&mut self, dt: &mut DrawTarget) {
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));
    }
}
