use crate::painter::window::Window;

use crate::layout::types::LayoutNode;

mod window;

pub fn paint(_layout_tree: &LayoutNode) {
    let mut window = Window::new("Ferrum", 800, 600);

    window.run();
}
