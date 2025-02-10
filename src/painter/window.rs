use minifb::Window as MinifbWindow;

pub(crate) struct Window {
    window: MinifbWindow,
}

impl Window {
    pub(crate) fn new(name: &str, width: usize, height: usize) -> Self {
        let window =
            MinifbWindow::new(name, width, height, minifb::WindowOptions::default()).unwrap();

        Self { window }
    }

    pub(crate) fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.window.update();
        }
    }
}
