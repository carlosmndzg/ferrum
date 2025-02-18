use raqote::DrawTarget;

use super::fonts_context::FontsContext;

pub(crate) mod draw_border;
pub(crate) mod draw_rectangle;
pub(crate) mod draw_text;

pub(crate) trait Command {
    fn execute(&self, dt: &mut DrawTarget, font_ctx: &mut FontsContext);
}
