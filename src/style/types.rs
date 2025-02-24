use core::fmt;
use std::collections::HashMap;

use crate::{
    layout::{
        box_types::{block::Block, inline::Inline},
        formatting_context::FormattingContext,
        types::BoxType,
    },
    style::properties::{
        background_color::BackgroundColor, border_color::BorderColor, border_style::BorderStyle,
        border_width::BorderWidth, color::Color, font_size::FontSize, font_weight::FontWeight,
        height::Height, line_height::LineHeight, margin_bottom::MarginBottom,
        margin_left::MarginLeft, margin_right::MarginRight, margin_top::MarginTop,
        padding_bottom::PaddingBottom, padding_left::PaddingLeft, padding_right::PaddingRight,
        padding_top::PaddingTop, text_align::TextAlign, width::Width, Property,
    },
    Node, NodeType,
};

use super::properties::display::Display;

#[derive(PartialEq)]
pub(crate) struct StyledNode<'a> {
    pub(crate) node: &'a Node,
    pub(crate) styles: Styles,
    pub(crate) children: Vec<StyledNode<'a>>,
}

impl fmt::Debug for StyledNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledNode")
            .field("node", &self.node_representation())
            .field("styles", &self.styles)
            .field("children", &self.children)
            .finish()
    }
}

macro_rules! generate_property_getter {
    ($name:ident, $property_type:ident) => {
        #[allow(unused)]
        pub(crate) fn $name(&self) -> &$property_type {
            let property_name = stringify!($name).replace('_', "-");

            if let Some(Property::$property_type(value)) = self.styles.get_property(&property_name)
            {
                return &value;
            }

            panic!(concat!(stringify!($property_type), " property not found"));
        }
    };
}

impl StyledNode<'_> {
    fn node_representation(&self) -> String {
        self.node_type_summary(&self.node.node_type)
    }

    fn node_type_summary(&self, node_type: &NodeType) -> String {
        match node_type {
            NodeType::Element(element) => element.tag_name().to_string(),
            NodeType::Text(text) => text.get().to_string(),
            _ => String::new(),
        }
    }

    pub(crate) fn is_empty_text_node(&self) -> bool {
        if let NodeType::Text(text) = &self.node.node_type {
            return text.get().trim().is_empty();
        }

        false
    }

    pub(crate) fn is_replaced_element(&self) -> bool {
        if let NodeType::Element(element) = &self.node.node_type {
            return element.tag_name() == "img";
        }

        false
    }

    pub(crate) fn has_display_none(&self) -> bool {
        self.display() == &Display::None
    }

    pub(crate) fn is_block_level(&self) -> bool {
        self.display() == &Display::Block
    }

    pub(crate) fn is_inline_level(&self) -> bool {
        self.display() == &Display::Inline
    }

    pub(crate) fn box_type(&self, formatting_context: FormattingContext) -> BoxType {
        if self.is_inline_level() {
            BoxType::Inline(Inline { node: self })
        } else {
            BoxType::Block(Block {
                node: self,
                formatting_context,
            })
        }
    }

    pub(crate) fn is_inline_in_block_context(
        &self,
        formatting_context: &FormattingContext,
    ) -> bool {
        self.is_inline_level()
            && !self.children.is_empty()
            && formatting_context == &FormattingContext::Block
    }

    pub(crate) fn children_displayed(&self) -> Vec<&StyledNode> {
        self.children
            .iter()
            .filter(|child| !child.has_display_none())
            .collect()
    }

    pub(crate) fn formatting_context(&self) -> FormattingContext {
        let children = self.children_displayed();

        if children.is_empty() {
            return FormattingContext::Block;
        }

        if children.iter().any(|child| child.is_block_level()) {
            FormattingContext::Block
        } else {
            FormattingContext::Inline
        }
    }

    generate_property_getter!(display, Display);
    generate_property_getter!(width, Width);
    generate_property_getter!(height, Height);
    generate_property_getter!(margin_top, MarginTop);
    generate_property_getter!(margin_right, MarginRight);
    generate_property_getter!(margin_bottom, MarginBottom);
    generate_property_getter!(margin_left, MarginLeft);
    generate_property_getter!(padding_top, PaddingTop);
    generate_property_getter!(padding_right, PaddingRight);
    generate_property_getter!(padding_bottom, PaddingBottom);
    generate_property_getter!(padding_left, PaddingLeft);
    generate_property_getter!(font_size, FontSize);
    generate_property_getter!(line_height, LineHeight);
    generate_property_getter!(background_color, BackgroundColor);
    generate_property_getter!(color, Color);
    generate_property_getter!(font_weight, FontWeight);
    generate_property_getter!(text_align, TextAlign);
    generate_property_getter!(border_width, BorderWidth);
    generate_property_getter!(border_color, BorderColor);
    generate_property_getter!(border_style, BorderStyle);
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Styles {
    properties: HashMap<String, Property>,
}

impl Styles {
    pub(crate) fn add_property(&mut self, property: Property) {
        let name = property.name().to_string();

        if self.properties.contains_key(&name) {
            return;
        }

        self.properties.insert(name, property);
    }

    pub(crate) fn get_property_clone(&self, name: &str) -> Option<Property> {
        if !self.properties.contains_key(name) {
            return None;
        }

        Some(self.properties.get(name).cloned().unwrap())
    }

    pub(crate) fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    pub(crate) fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Unit {
    Px,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub(crate) struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: f32,
}

impl Rgb {
    pub(crate) fn convert_keyword_to_rgb(keyword: &str) -> Option<Rgb> {
        match keyword {
            "black" => Rgb::wrap_color(0, 0, 0, 1.0),
            "silver" => Rgb::wrap_color(192, 192, 192, 1.0),
            "gray" => Rgb::wrap_color(128, 128, 128, 1.0),
            "white" => Rgb::wrap_color(255, 255, 255, 1.0),
            "maroon" => Rgb::wrap_color(128, 0, 0, 1.0),
            "red" => Rgb::wrap_color(255, 0, 0, 1.0),
            "purple" => Rgb::wrap_color(128, 0, 128, 1.0),
            "fuchsia" => Rgb::wrap_color(255, 0, 255, 1.0),
            "green" => Rgb::wrap_color(0, 128, 0, 1.0),
            "lime" => Rgb::wrap_color(0, 255, 0, 1.0),
            "olive" => Rgb::wrap_color(128, 128, 0, 1.0),
            "yellow" => Rgb::wrap_color(255, 255, 0, 1.0),
            "navy" => Rgb::wrap_color(0, 0, 128, 1.0),
            "blue" => Rgb::wrap_color(0, 0, 255, 1.0),
            "teal" => Rgb::wrap_color(0, 128, 128, 1.0),
            "aqua" => Rgb::wrap_color(0, 255, 255, 1.0),
            "orange" => Rgb::wrap_color(255, 165, 0, 1.0),
            "aliceblue" => Rgb::wrap_color(240, 248, 255, 1.0),
            "antiquewhite" => Rgb::wrap_color(250, 235, 215, 1.0),
            "aquamarine" => Rgb::wrap_color(127, 255, 212, 1.0),
            "azure" => Rgb::wrap_color(240, 255, 255, 1.0),
            "beige" => Rgb::wrap_color(245, 245, 220, 1.0),
            "bisque" => Rgb::wrap_color(255, 228, 196, 1.0),
            "blanchedalmond" => Rgb::wrap_color(255, 235, 205, 1.0),
            "blueviolet" => Rgb::wrap_color(138, 43, 226, 1.0),
            "brown" => Rgb::wrap_color(165, 42, 42, 1.0),
            "burlywood" => Rgb::wrap_color(222, 184, 135, 1.0),
            "cadetblue" => Rgb::wrap_color(95, 158, 160, 1.0),
            "chartreuse" => Rgb::wrap_color(127, 255, 0, 1.0),
            "chocolate" => Rgb::wrap_color(210, 105, 30, 1.0),
            "coral" => Rgb::wrap_color(255, 127, 80, 1.0),
            "cornflowerblue" => Rgb::wrap_color(100, 149, 237, 1.0),
            "cornsilk" => Rgb::wrap_color(255, 248, 220, 1.0),
            "crimson" => Rgb::wrap_color(220, 20, 60, 1.0),
            "cyan" => Rgb::wrap_color(0, 255, 255, 1.0),
            "darkblue" => Rgb::wrap_color(0, 0, 139, 1.0),
            "darkcyan" => Rgb::wrap_color(0, 139, 139, 1.0),
            "darkgoldenrod" => Rgb::wrap_color(184, 134, 11, 1.0),
            "darkgray" => Rgb::wrap_color(169, 169, 169, 1.0),
            "darkgreen" => Rgb::wrap_color(0, 100, 0, 1.0),
            "darkgrey" => Rgb::wrap_color(169, 169, 169, 1.0),
            "darkkhaki" => Rgb::wrap_color(189, 183, 107, 1.0),
            "darkmagenta" => Rgb::wrap_color(139, 0, 139, 1.0),
            "darkolivegreen" => Rgb::wrap_color(85, 107, 47, 1.0),
            "darkorange" => Rgb::wrap_color(255, 140, 0, 1.0),
            "darkorchid" => Rgb::wrap_color(153, 50, 204, 1.0),
            "darkred" => Rgb::wrap_color(139, 0, 0, 1.0),
            "darksalmon" => Rgb::wrap_color(233, 150, 122, 1.0),
            "darkseagreen" => Rgb::wrap_color(143, 188, 143, 1.0),
            "darkslateblue" => Rgb::wrap_color(72, 61, 139, 1.0),
            "darkslategray" => Rgb::wrap_color(47, 79, 79, 1.0),
            "darkslategrey" => Rgb::wrap_color(47, 79, 79, 1.0),
            "darkturquoise" => Rgb::wrap_color(0, 206, 209, 1.0),
            "darkviolet" => Rgb::wrap_color(148, 0, 211, 1.0),
            "deeppink" => Rgb::wrap_color(255, 20, 147, 1.0),
            "deepskyblue" => Rgb::wrap_color(0, 191, 255, 1.0),
            "dimgray" => Rgb::wrap_color(105, 105, 105, 1.0),
            "dimgrey" => Rgb::wrap_color(105, 105, 105, 1.0),
            "dodgerblue" => Rgb::wrap_color(30, 144, 255, 1.0),
            "firebrick" => Rgb::wrap_color(178, 34, 34, 1.0),
            "floralwhite" => Rgb::wrap_color(255, 250, 240, 1.0),
            "forestgreen" => Rgb::wrap_color(34, 139, 34, 1.0),
            "gainsboro" => Rgb::wrap_color(220, 220, 220, 1.0),
            "ghostwhite" => Rgb::wrap_color(248, 248, 255, 1.0),
            "gold" => Rgb::wrap_color(255, 215, 0, 1.0),
            "goldenrod" => Rgb::wrap_color(218, 165, 32, 1.0),
            "greenyellow" => Rgb::wrap_color(173, 255, 47, 1.0),
            "grey" => Rgb::wrap_color(128, 128, 128, 1.0),
            "honeydew" => Rgb::wrap_color(240, 255, 240, 1.0),
            "hotpink" => Rgb::wrap_color(255, 105, 180, 1.0),
            "indianred" => Rgb::wrap_color(205, 92, 92, 1.0),
            "indigo" => Rgb::wrap_color(75, 0, 130, 1.0),
            "ivory" => Rgb::wrap_color(255, 255, 240, 1.0),
            "khaki" => Rgb::wrap_color(240, 230, 140, 1.0),
            "lavender" => Rgb::wrap_color(230, 230, 250, 1.0),
            "lavenderblush" => Rgb::wrap_color(255, 240, 245, 1.0),
            "lawngreen" => Rgb::wrap_color(124, 252, 0, 1.0),
            "lemonchiffon" => Rgb::wrap_color(255, 250, 205, 1.0),
            "lightblue" => Rgb::wrap_color(173, 216, 230, 1.0),
            "lightcoral" => Rgb::wrap_color(240, 128, 128, 1.0),
            "lightcyan" => Rgb::wrap_color(224, 255, 255, 1.0),
            "lightgoldenrodyellow" => Rgb::wrap_color(250, 250, 210, 1.0),
            "lightgray" => Rgb::wrap_color(211, 211, 211, 1.0),
            "lightgreen" => Rgb::wrap_color(144, 238, 144, 1.0),
            "lightgrey" => Rgb::wrap_color(211, 211, 211, 1.0),
            "lightpink" => Rgb::wrap_color(255, 182, 193, 1.0),
            "lightsalmon" => Rgb::wrap_color(255, 160, 122, 1.0),
            "lightseagreen" => Rgb::wrap_color(32, 178, 170, 1.0),
            "lightskyblue" => Rgb::wrap_color(135, 206, 250, 1.0),
            "lightslategray" => Rgb::wrap_color(119, 136, 153, 1.0),
            "lightslategrey" => Rgb::wrap_color(119, 136, 153, 1.0),
            "lightsteelblue" => Rgb::wrap_color(176, 196, 222, 1.0),
            "lightyellow" => Rgb::wrap_color(255, 255, 224, 1.0),
            "limegreen" => Rgb::wrap_color(50, 205, 50, 1.0),
            "linen" => Rgb::wrap_color(250, 240, 230, 1.0),
            "magenta" => Rgb::wrap_color(255, 0, 255, 1.0),
            "mediumaquamarine" => Rgb::wrap_color(102, 205, 170, 1.0),
            "mediumblue" => Rgb::wrap_color(0, 0, 205, 1.0),
            "mediumorchid" => Rgb::wrap_color(186, 85, 211, 1.0),
            "mediumpurple" => Rgb::wrap_color(147, 112, 219, 1.0),
            "mediumseagreen" => Rgb::wrap_color(60, 179, 113, 1.0),
            "mediumslateblue" => Rgb::wrap_color(123, 104, 238, 1.0),
            "mediumspringgreen" => Rgb::wrap_color(0, 250, 154, 1.0),
            "mediumturquoise" => Rgb::wrap_color(72, 209, 204, 1.0),
            "mediumvioletred" => Rgb::wrap_color(199, 21, 133, 1.0),
            "midnightblue" => Rgb::wrap_color(25, 25, 112, 1.0),
            "mintcream" => Rgb::wrap_color(245, 255, 250, 1.0),
            "mistyrose" => Rgb::wrap_color(255, 228, 225, 1.0),
            "moccasin" => Rgb::wrap_color(255, 228, 181, 1.0),
            "navajowhite" => Rgb::wrap_color(255, 222, 173, 1.0),
            "oldlace" => Rgb::wrap_color(253, 245, 230, 1.0),
            "olivedrab" => Rgb::wrap_color(107, 142, 35, 1.0),
            "orangered" => Rgb::wrap_color(255, 69, 0, 1.0),
            "orchid" => Rgb::wrap_color(218, 112, 214, 1.0),
            "palegoldenrod" => Rgb::wrap_color(238, 232, 170, 1.0),
            "palegreen" => Rgb::wrap_color(152, 251, 152, 1.0),
            "paleturquoise" => Rgb::wrap_color(175, 238, 238, 1.0),
            "palevioletred" => Rgb::wrap_color(219, 112, 147, 1.0),
            "papayawhip" => Rgb::wrap_color(255, 239, 213, 1.0),
            "peachpuff" => Rgb::wrap_color(255, 218, 185, 1.0),
            "peru" => Rgb::wrap_color(205, 133, 63, 1.0),
            "pink" => Rgb::wrap_color(255, 192, 203, 1.0),
            "plum" => Rgb::wrap_color(221, 160, 221, 1.0),
            "powderblue" => Rgb::wrap_color(176, 224, 230, 1.0),
            "rebeccapurple" => Rgb::wrap_color(102, 51, 153, 1.0),
            "rosybrown" => Rgb::wrap_color(188, 143, 143, 1.0),
            "royalblue" => Rgb::wrap_color(65, 105, 225, 1.0),
            "saddlebrown" => Rgb::wrap_color(139, 69, 19, 1.0),
            "salmon" => Rgb::wrap_color(250, 128, 114, 1.0),
            "sandybrown" => Rgb::wrap_color(244, 164, 96, 1.0),
            "seagreen" => Rgb::wrap_color(46, 139, 87, 1.0),
            "seashell" => Rgb::wrap_color(255, 245, 238, 1.0),
            "sienna" => Rgb::wrap_color(160, 82, 45, 1.0),
            "skyblue" => Rgb::wrap_color(135, 206, 235, 1.0),
            "slateblue" => Rgb::wrap_color(106, 90, 205, 1.0),
            "slategray" => Rgb::wrap_color(112, 128, 144, 1.0),
            "slategrey" => Rgb::wrap_color(112, 128, 144, 1.0),
            "snow" => Rgb::wrap_color(255, 250, 250, 1.0),
            "springgreen" => Rgb::wrap_color(0, 255, 127, 1.0),
            "steelblue" => Rgb::wrap_color(70, 130, 180, 1.0),
            "tan" => Rgb::wrap_color(210, 180, 140, 1.0),
            "thistle" => Rgb::wrap_color(216, 191, 216, 1.0),
            "tomato" => Rgb::wrap_color(255, 99, 71, 1.0),
            "turquoise" => Rgb::wrap_color(64, 224, 208, 1.0),
            "violet" => Rgb::wrap_color(238, 130, 238, 1.0),
            "wheat" => Rgb::wrap_color(245, 222, 179, 1.0),
            "whitesmoke" => Rgb::wrap_color(245, 245, 245, 1.0),
            "yellowgreen" => Rgb::wrap_color(154, 205, 50, 1.0),
            "transparent" => Rgb::wrap_color(0, 0, 0, 0.0),
            _ => None,
        }
    }

    pub(crate) fn wrap_color(r: u8, g: u8, b: u8, a: f32) -> Option<Rgb> {
        Some(Rgb { r, g, b, a })
    }
}

impl From<&crate::css::types::Color> for Option<Rgb> {
    fn from(color: &crate::css::types::Color) -> Self {
        let (r, g, b) = (color.r, color.g, color.b);

        Some(Rgb { r, g, b, a: 1.0 })
    }
}
