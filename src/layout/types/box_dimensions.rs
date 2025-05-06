#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub(crate) struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct EdgeSizes {
    pub(crate) top: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
    pub(crate) right: f32,
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct BoxDimensions {
    pub(crate) content: Rectangle,
    pub(crate) padding: EdgeSizes,
    pub(crate) border: EdgeSizes,
    pub(crate) margin: EdgeSizes,
}

impl BoxDimensions {
    pub(crate) fn border_box(&self) -> Rectangle {
        let mut border_box = self.content.clone();

        border_box.x -= self.padding.left + self.border.left;
        border_box.y -= self.padding.top + self.border.top;
        border_box.width +=
            self.padding.left + self.padding.right + self.border.left + self.border.right;
        border_box.height +=
            self.padding.top + self.padding.bottom + self.border.top + self.border.bottom;

        border_box
    }

    pub(crate) fn padding_box(&self) -> Rectangle {
        let mut padding_box = self.content.clone();

        padding_box.x -= self.padding.left;
        padding_box.y -= self.padding.top;
        padding_box.width += self.padding.left + self.padding.right;
        padding_box.height += self.padding.top + self.padding.bottom;

        padding_box
    }
}
