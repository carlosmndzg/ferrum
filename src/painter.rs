use crate::painter::window::Window;

use crate::layout::types::LayoutNode;

mod command_list;
mod commands;
mod window;

pub fn paint(root: &LayoutNode) {
    let mut window = Window::new("Ferrum", 800, 600);

    window.run(root);
}
