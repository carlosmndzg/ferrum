use std::path::Path;

use crate::painter::window::Window;

use crate::style::types::StyledNode;

mod command_list;
mod commands;
mod fonts_context;
mod window;

pub fn paint(root: &StyledNode, file_path: &Path) {
    let mut window = Window::new("Ferrum", 800, 600);

    window.run(root, file_path);
}
