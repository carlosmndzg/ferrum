use core::fmt;
use std::collections::HashMap;

use crate::{style::properties::Property, Element, Node, NodeType};

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

impl StyledNode<'_> {
    fn node_representation(&self) -> String {
        self.node_type_summary(&self.node.node_type)
    }

    fn node_type_summary(&self, node_type: &NodeType) -> String {
        match node_type {
            NodeType::Element(Element { tag_name, .. }) => tag_name.clone(),
            NodeType::Text(text) => text.text.clone(),
            _ => String::new(),
        }
    }

    pub(crate) fn display(&self) -> &Display {
        if let Some(Property::Display(display)) = self.styles.get_property("display") {
            return display;
        }

        panic!("Display property not found");
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
}

impl Rgb {
    pub(crate) fn convert_keyword_to_rgb(keyword: &str) -> Option<Rgb> {
        match keyword {
            "black" => Rgb::wrap_color(0, 0, 0),
            "silver" => Rgb::wrap_color(192, 192, 192),
            "gray" => Rgb::wrap_color(128, 128, 128),
            "white" => Rgb::wrap_color(255, 255, 255),
            "maroon" => Rgb::wrap_color(128, 0, 0),
            "red" => Rgb::wrap_color(255, 0, 0),
            "purple" => Rgb::wrap_color(128, 0, 128),
            "fuchsia" => Rgb::wrap_color(255, 0, 255),
            "green" => Rgb::wrap_color(0, 128, 0),
            "lime" => Rgb::wrap_color(0, 255, 0),
            "olive" => Rgb::wrap_color(128, 128, 0),
            "yellow" => Rgb::wrap_color(255, 255, 0),
            "navy" => Rgb::wrap_color(0, 0, 128),
            "blue" => Rgb::wrap_color(0, 0, 255),
            "teal" => Rgb::wrap_color(0, 128, 128),
            "aqua" => Rgb::wrap_color(0, 255, 255),
            "orange" => Rgb::wrap_color(255, 165, 0),
            "aliceblue" => Rgb::wrap_color(240, 248, 255),
            "antiquewhite" => Rgb::wrap_color(250, 235, 215),
            "aquamarine" => Rgb::wrap_color(127, 255, 212),
            "azure" => Rgb::wrap_color(240, 255, 255),
            "beige" => Rgb::wrap_color(245, 245, 220),
            "bisque" => Rgb::wrap_color(255, 228, 196),
            "blanchedalmond" => Rgb::wrap_color(255, 235, 205),
            "blueviolet" => Rgb::wrap_color(138, 43, 226),
            "brown" => Rgb::wrap_color(165, 42, 42),
            "burlywood" => Rgb::wrap_color(222, 184, 135),
            "cadetblue" => Rgb::wrap_color(95, 158, 160),
            "chartreuse" => Rgb::wrap_color(127, 255, 0),
            "chocolate" => Rgb::wrap_color(210, 105, 30),
            "coral" => Rgb::wrap_color(255, 127, 80),
            "cornflowerblue" => Rgb::wrap_color(100, 149, 237),
            "cornsilk" => Rgb::wrap_color(255, 248, 220),
            "crimson" => Rgb::wrap_color(220, 20, 60),
            "cyan" => Rgb::wrap_color(0, 255, 255),
            "darkblue" => Rgb::wrap_color(0, 0, 139),
            "darkcyan" => Rgb::wrap_color(0, 139, 139),
            "darkgoldenrod" => Rgb::wrap_color(184, 134, 11),
            "darkgray" => Rgb::wrap_color(169, 169, 169),
            "darkgreen" => Rgb::wrap_color(0, 100, 0),
            "darkgrey" => Rgb::wrap_color(169, 169, 169),
            "darkkhaki" => Rgb::wrap_color(189, 183, 107),
            "darkmagenta" => Rgb::wrap_color(139, 0, 139),
            "darkolivegreen" => Rgb::wrap_color(85, 107, 47),
            "darkorange" => Rgb::wrap_color(255, 140, 0),
            "darkorchid" => Rgb::wrap_color(153, 50, 204),
            "darkred" => Rgb::wrap_color(139, 0, 0),
            "darksalmon" => Rgb::wrap_color(233, 150, 122),
            "darkseagreen" => Rgb::wrap_color(143, 188, 143),
            "darkslateblue" => Rgb::wrap_color(72, 61, 139),
            "darkslategray" => Rgb::wrap_color(47, 79, 79),
            "darkslategrey" => Rgb::wrap_color(47, 79, 79),
            "darkturquoise" => Rgb::wrap_color(0, 206, 209),
            "darkviolet" => Rgb::wrap_color(148, 0, 211),
            "deeppink" => Rgb::wrap_color(255, 20, 147),
            "deepskyblue" => Rgb::wrap_color(0, 191, 255),
            "dimgray" => Rgb::wrap_color(105, 105, 105),
            "dimgrey" => Rgb::wrap_color(105, 105, 105),
            "dodgerblue" => Rgb::wrap_color(30, 144, 255),
            "firebrick" => Rgb::wrap_color(178, 34, 34),
            "floralwhite" => Rgb::wrap_color(255, 250, 240),
            "forestgreen" => Rgb::wrap_color(34, 139, 34),
            "gainsboro" => Rgb::wrap_color(220, 220, 220),
            "ghostwhite" => Rgb::wrap_color(248, 248, 255),
            "gold" => Rgb::wrap_color(255, 215, 0),
            "goldenrod" => Rgb::wrap_color(218, 165, 32),
            "greenyellow" => Rgb::wrap_color(173, 255, 47),
            "grey" => Rgb::wrap_color(128, 128, 128),
            "honeydew" => Rgb::wrap_color(240, 255, 240),
            "hotpink" => Rgb::wrap_color(255, 105, 180),
            "indianred" => Rgb::wrap_color(205, 92, 92),
            "indigo" => Rgb::wrap_color(75, 0, 130),
            "ivory" => Rgb::wrap_color(255, 255, 240),
            "khaki" => Rgb::wrap_color(240, 230, 140),
            "lavender" => Rgb::wrap_color(230, 230, 250),
            "lavenderblush" => Rgb::wrap_color(255, 240, 245),
            "lawngreen" => Rgb::wrap_color(124, 252, 0),
            "lemonchiffon" => Rgb::wrap_color(255, 250, 205),
            "lightblue" => Rgb::wrap_color(173, 216, 230),
            "lightcoral" => Rgb::wrap_color(240, 128, 128),
            "lightcyan" => Rgb::wrap_color(224, 255, 255),
            "lightgoldenrodyellow" => Rgb::wrap_color(250, 250, 210),
            "lightgray" => Rgb::wrap_color(211, 211, 211),
            "lightgreen" => Rgb::wrap_color(144, 238, 144),
            "lightgrey" => Rgb::wrap_color(211, 211, 211),
            "lightpink" => Rgb::wrap_color(255, 182, 193),
            "lightsalmon" => Rgb::wrap_color(255, 160, 122),
            "lightseagreen" => Rgb::wrap_color(32, 178, 170),
            "lightskyblue" => Rgb::wrap_color(135, 206, 250),
            "lightslategray" => Rgb::wrap_color(119, 136, 153),
            "lightslategrey" => Rgb::wrap_color(119, 136, 153),
            "lightsteelblue" => Rgb::wrap_color(176, 196, 222),
            "lightyellow" => Rgb::wrap_color(255, 255, 224),
            "limegreen" => Rgb::wrap_color(50, 205, 50),
            "linen" => Rgb::wrap_color(250, 240, 230),
            "magenta" => Rgb::wrap_color(255, 0, 255),
            "mediumaquamarine" => Rgb::wrap_color(102, 205, 170),
            "mediumblue" => Rgb::wrap_color(0, 0, 205),
            "mediumorchid" => Rgb::wrap_color(186, 85, 211),
            "mediumpurple" => Rgb::wrap_color(147, 112, 219),
            "mediumseagreen" => Rgb::wrap_color(60, 179, 113),
            "mediumslateblue" => Rgb::wrap_color(123, 104, 238),
            "mediumspringgreen" => Rgb::wrap_color(0, 250, 154),
            "mediumturquoise" => Rgb::wrap_color(72, 209, 204),
            "mediumvioletred" => Rgb::wrap_color(199, 21, 133),
            "midnightblue" => Rgb::wrap_color(25, 25, 112),
            "mintcream" => Rgb::wrap_color(245, 255, 250),
            "mistyrose" => Rgb::wrap_color(255, 228, 225),
            "moccasin" => Rgb::wrap_color(255, 228, 181),
            "navajowhite" => Rgb::wrap_color(255, 222, 173),
            "oldlace" => Rgb::wrap_color(253, 245, 230),
            "olivedrab" => Rgb::wrap_color(107, 142, 35),
            "orangered" => Rgb::wrap_color(255, 69, 0),
            "orchid" => Rgb::wrap_color(218, 112, 214),
            "palegoldenrod" => Rgb::wrap_color(238, 232, 170),
            "palegreen" => Rgb::wrap_color(152, 251, 152),
            "paleturquoise" => Rgb::wrap_color(175, 238, 238),
            "palevioletred" => Rgb::wrap_color(219, 112, 147),
            "papayawhip" => Rgb::wrap_color(255, 239, 213),
            "peachpuff" => Rgb::wrap_color(255, 218, 185),
            "peru" => Rgb::wrap_color(205, 133, 63),
            "pink" => Rgb::wrap_color(255, 192, 203),
            "plum" => Rgb::wrap_color(221, 160, 221),
            "powderblue" => Rgb::wrap_color(176, 224, 230),
            "rebeccapurple" => Rgb::wrap_color(102, 51, 153),
            "rosybrown" => Rgb::wrap_color(188, 143, 143),
            "royalblue" => Rgb::wrap_color(65, 105, 225),
            "saddlebrown" => Rgb::wrap_color(139, 69, 19),
            "salmon" => Rgb::wrap_color(250, 128, 114),
            "sandybrown" => Rgb::wrap_color(244, 164, 96),
            "seagreen" => Rgb::wrap_color(46, 139, 87),
            "seashell" => Rgb::wrap_color(255, 245, 238),
            "sienna" => Rgb::wrap_color(160, 82, 45),
            "skyblue" => Rgb::wrap_color(135, 206, 235),
            "slateblue" => Rgb::wrap_color(106, 90, 205),
            "slategray" => Rgb::wrap_color(112, 128, 144),
            "slategrey" => Rgb::wrap_color(112, 128, 144),
            "snow" => Rgb::wrap_color(255, 250, 250),
            "springgreen" => Rgb::wrap_color(0, 255, 127),
            "steelblue" => Rgb::wrap_color(70, 130, 180),
            "tan" => Rgb::wrap_color(210, 180, 140),
            "thistle" => Rgb::wrap_color(216, 191, 216),
            "tomato" => Rgb::wrap_color(255, 99, 71),
            "turquoise" => Rgb::wrap_color(64, 224, 208),
            "violet" => Rgb::wrap_color(238, 130, 238),
            "wheat" => Rgb::wrap_color(245, 222, 179),
            "whitesmoke" => Rgb::wrap_color(245, 245, 245),
            "yellowgreen" => Rgb::wrap_color(154, 205, 50),
            _ => None,
        }
    }

    pub(crate) fn wrap_color(r: u8, g: u8, b: u8) -> Option<Rgb> {
        Some(Rgb { r, g, b })
    }
}

impl From<&crate::css::types::Color> for Option<Rgb> {
    fn from(color: &crate::css::types::Color) -> Self {
        let (r, g, b) = (color.r, color.g, color.b);

        Some(Rgb { r, g, b })
    }
}
