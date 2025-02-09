#[derive(Debug)]
#[allow(unused)]
pub(crate) struct LayoutNode {
    pub(crate) box_dimensions: BoxDimensions,
    pub(crate) children: Vec<LayoutNode>,
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct BoxDimensions {
    pub(crate) content: Rectangle,
    pub(crate) padding: EdgeSizes,
    pub(crate) border: EdgeSizes,
    pub(crate) margin: EdgeSizes,
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct EdgeSizes {
    pub(crate) top: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
    pub(crate) right: f32,
}
