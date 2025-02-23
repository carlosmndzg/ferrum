use std::path::Path;

use crate::painter::window::Window;

use crate::layout::types::LayoutNode;

mod command_list;
mod commands;
mod fonts_context;
mod window;

pub fn paint(root: &LayoutNode, file_path: &Path) {
    let mut window = Window::new("Ferrum", 800, 600);

    window.run(root, file_path);
}
