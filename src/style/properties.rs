use std::any::Any;

use crate::css::types::{Declaration, Value};

pub(crate) struct PropertyFactory;

impl PropertyFactory {
    pub(crate) fn create_property(declaration: &Declaration) -> Option<Box<dyn Property>> {
        match declaration.name.as_str() {
            "color" => Color::maybe_new(&declaration.value),
            _ => None,
        }
    }
}

pub(crate) trait Property: std::fmt::Debug + Any {
    fn maybe_new(value: &Value) -> Option<Box<dyn Property>>
    where
        Self: Sized;

    fn name(&self) -> &str;

    #[allow(unused)]
    fn as_any(&self) -> &dyn Any;
}

//TODO Remove unsued attribute
#[derive(Debug)]
#[allow(unused)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Property for Color {
    fn maybe_new(value: &Value) -> Option<Box<dyn Property>> {
        if let Value::Keyword(keyword) = value {
            return Color::convert_keyword_to_color(keyword);
        }

        if let Value::ColorValue(color) = value {
            return color.into();
        }

        None
    }

    fn name(&self) -> &str {
        "color"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Color {
    fn convert_keyword_to_color(keyword: &str) -> Option<Box<dyn Property>> {
        match keyword {
            "black" => Color::wrap_color(0, 0, 0),
            "silver" => Color::wrap_color(192, 192, 192),
            "gray" => Color::wrap_color(128, 128, 128),
            "white" => Color::wrap_color(255, 255, 255),
            "maroon" => Color::wrap_color(128, 0, 0),
            "red" => Color::wrap_color(255, 0, 0),
            "purple" => Color::wrap_color(128, 0, 128),
            "fuchsia" => Color::wrap_color(255, 0, 255),
            "green" => Color::wrap_color(0, 128, 0),
            "lime" => Color::wrap_color(0, 255, 0),
            "olive" => Color::wrap_color(128, 128, 0),
            "yellow" => Color::wrap_color(255, 255, 0),
            "navy" => Color::wrap_color(0, 0, 128),
            "blue" => Color::wrap_color(0, 0, 255),
            "teal" => Color::wrap_color(0, 128, 128),
            "aqua" => Color::wrap_color(0, 255, 255),
            "orange" => Color::wrap_color(255, 165, 0),
            "aliceblue" => Color::wrap_color(240, 248, 255),
            "antiquewhite" => Color::wrap_color(250, 235, 215),
            "aquamarine" => Color::wrap_color(127, 255, 212),
            "azure" => Color::wrap_color(240, 255, 255),
            "beige" => Color::wrap_color(245, 245, 220),
            "bisque" => Color::wrap_color(255, 228, 196),
            "blanchedalmond" => Color::wrap_color(255, 235, 205),
            "blueviolet" => Color::wrap_color(138, 43, 226),
            "brown" => Color::wrap_color(165, 42, 42),
            "burlywood" => Color::wrap_color(222, 184, 135),
            "cadetblue" => Color::wrap_color(95, 158, 160),
            "chartreuse" => Color::wrap_color(127, 255, 0),
            "chocolate" => Color::wrap_color(210, 105, 30),
            "coral" => Color::wrap_color(255, 127, 80),
            "cornflowerblue" => Color::wrap_color(100, 149, 237),
            "cornsilk" => Color::wrap_color(255, 248, 220),
            "crimson" => Color::wrap_color(220, 20, 60),
            "cyan" => Color::wrap_color(0, 255, 255),
            "darkblue" => Color::wrap_color(0, 0, 139),
            "darkcyan" => Color::wrap_color(0, 139, 139),
            "darkgoldenrod" => Color::wrap_color(184, 134, 11),
            "darkgray" => Color::wrap_color(169, 169, 169),
            "darkgreen" => Color::wrap_color(0, 100, 0),
            "darkgrey" => Color::wrap_color(169, 169, 169),
            "darkkhaki" => Color::wrap_color(189, 183, 107),
            "darkmagenta" => Color::wrap_color(139, 0, 139),
            "darkolivegreen" => Color::wrap_color(85, 107, 47),
            "darkorange" => Color::wrap_color(255, 140, 0),
            "darkorchid" => Color::wrap_color(153, 50, 204),
            "darkred" => Color::wrap_color(139, 0, 0),
            "darksalmon" => Color::wrap_color(233, 150, 122),
            "darkseagreen" => Color::wrap_color(143, 188, 143),
            "darkslateblue" => Color::wrap_color(72, 61, 139),
            "darkslategray" => Color::wrap_color(47, 79, 79),
            "darkslategrey" => Color::wrap_color(47, 79, 79),
            "darkturquoise" => Color::wrap_color(0, 206, 209),
            "darkviolet" => Color::wrap_color(148, 0, 211),
            "deeppink" => Color::wrap_color(255, 20, 147),
            "deepskyblue" => Color::wrap_color(0, 191, 255),
            "dimgray" => Color::wrap_color(105, 105, 105),
            "dimgrey" => Color::wrap_color(105, 105, 105),
            "dodgerblue" => Color::wrap_color(30, 144, 255),
            "firebrick" => Color::wrap_color(178, 34, 34),
            "floralwhite" => Color::wrap_color(255, 250, 240),
            "forestgreen" => Color::wrap_color(34, 139, 34),
            "gainsboro" => Color::wrap_color(220, 220, 220),
            "ghostwhite" => Color::wrap_color(248, 248, 255),
            "gold" => Color::wrap_color(255, 215, 0),
            "goldenrod" => Color::wrap_color(218, 165, 32),
            "greenyellow" => Color::wrap_color(173, 255, 47),
            "grey" => Color::wrap_color(128, 128, 128),
            "honeydew" => Color::wrap_color(240, 255, 240),
            "hotpink" => Color::wrap_color(255, 105, 180),
            "indianred" => Color::wrap_color(205, 92, 92),
            "indigo" => Color::wrap_color(75, 0, 130),
            "ivory" => Color::wrap_color(255, 255, 240),
            "khaki" => Color::wrap_color(240, 230, 140),
            "lavender" => Color::wrap_color(230, 230, 250),
            "lavenderblush" => Color::wrap_color(255, 240, 245),
            "lawngreen" => Color::wrap_color(124, 252, 0),
            "lemonchiffon" => Color::wrap_color(255, 250, 205),
            "lightblue" => Color::wrap_color(173, 216, 230),
            "lightcoral" => Color::wrap_color(240, 128, 128),
            "lightcyan" => Color::wrap_color(224, 255, 255),
            "lightgoldenrodyellow" => Color::wrap_color(250, 250, 210),
            "lightgray" => Color::wrap_color(211, 211, 211),
            "lightgreen" => Color::wrap_color(144, 238, 144),
            "lightgrey" => Color::wrap_color(211, 211, 211),
            "lightpink" => Color::wrap_color(255, 182, 193),
            "lightsalmon" => Color::wrap_color(255, 160, 122),
            "lightseagreen" => Color::wrap_color(32, 178, 170),
            "lightskyblue" => Color::wrap_color(135, 206, 250),
            "lightslategray" => Color::wrap_color(119, 136, 153),
            "lightslategrey" => Color::wrap_color(119, 136, 153),
            "lightsteelblue" => Color::wrap_color(176, 196, 222),
            "lightyellow" => Color::wrap_color(255, 255, 224),
            "limegreen" => Color::wrap_color(50, 205, 50),
            "linen" => Color::wrap_color(250, 240, 230),
            "magenta" => Color::wrap_color(255, 0, 255),
            "mediumaquamarine" => Color::wrap_color(102, 205, 170),
            "mediumblue" => Color::wrap_color(0, 0, 205),
            "mediumorchid" => Color::wrap_color(186, 85, 211),
            "mediumpurple" => Color::wrap_color(147, 112, 219),
            "mediumseagreen" => Color::wrap_color(60, 179, 113),
            "mediumslateblue" => Color::wrap_color(123, 104, 238),
            "mediumspringgreen" => Color::wrap_color(0, 250, 154),
            "mediumturquoise" => Color::wrap_color(72, 209, 204),
            "mediumvioletred" => Color::wrap_color(199, 21, 133),
            "midnightblue" => Color::wrap_color(25, 25, 112),
            "mintcream" => Color::wrap_color(245, 255, 250),
            "mistyrose" => Color::wrap_color(255, 228, 225),
            "moccasin" => Color::wrap_color(255, 228, 181),
            "navajowhite" => Color::wrap_color(255, 222, 173),
            "oldlace" => Color::wrap_color(253, 245, 230),
            "olivedrab" => Color::wrap_color(107, 142, 35),
            "orangered" => Color::wrap_color(255, 69, 0),
            "orchid" => Color::wrap_color(218, 112, 214),
            "palegoldenrod" => Color::wrap_color(238, 232, 170),
            "palegreen" => Color::wrap_color(152, 251, 152),
            "paleturquoise" => Color::wrap_color(175, 238, 238),
            "palevioletred" => Color::wrap_color(219, 112, 147),
            "papayawhip" => Color::wrap_color(255, 239, 213),
            "peachpuff" => Color::wrap_color(255, 218, 185),
            "peru" => Color::wrap_color(205, 133, 63),
            "pink" => Color::wrap_color(255, 192, 203),
            "plum" => Color::wrap_color(221, 160, 221),
            "powderblue" => Color::wrap_color(176, 224, 230),
            "rebeccapurple" => Color::wrap_color(102, 51, 153),
            "rosybrown" => Color::wrap_color(188, 143, 143),
            "royalblue" => Color::wrap_color(65, 105, 225),
            "saddlebrown" => Color::wrap_color(139, 69, 19),
            "salmon" => Color::wrap_color(250, 128, 114),
            "sandybrown" => Color::wrap_color(244, 164, 96),
            "seagreen" => Color::wrap_color(46, 139, 87),
            "seashell" => Color::wrap_color(255, 245, 238),
            "sienna" => Color::wrap_color(160, 82, 45),
            "skyblue" => Color::wrap_color(135, 206, 235),
            "slateblue" => Color::wrap_color(106, 90, 205),
            "slategray" => Color::wrap_color(112, 128, 144),
            "slategrey" => Color::wrap_color(112, 128, 144),
            "snow" => Color::wrap_color(255, 250, 250),
            "springgreen" => Color::wrap_color(0, 255, 127),
            "steelblue" => Color::wrap_color(70, 130, 180),
            "tan" => Color::wrap_color(210, 180, 140),
            "thistle" => Color::wrap_color(216, 191, 216),
            "tomato" => Color::wrap_color(255, 99, 71),
            "turquoise" => Color::wrap_color(64, 224, 208),
            "violet" => Color::wrap_color(238, 130, 238),
            "wheat" => Color::wrap_color(245, 222, 179),
            "whitesmoke" => Color::wrap_color(245, 245, 245),
            "yellowgreen" => Color::wrap_color(154, 205, 50),
            _ => None,
        }
    }

    fn wrap_color(r: u8, g: u8, b: u8) -> Option<Box<dyn Property>> {
        Some(Box::new(Color { r, g, b }))
    }
}

impl From<&crate::css::types::Color> for Option<Box<dyn Property>> {
    fn from(color: &crate::css::types::Color) -> Self {
        let (r, g, b) = (color.r, color.g, color.b);

        Some(Box::new(Color { r, g, b }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_property() {
        let declaration = Declaration {
            name: "color".to_string(),
            value: Value::Keyword("red".to_string()),
        };

        let property = PropertyFactory::create_property(&declaration);

        if let Some(property) = property {
            if let Some(color) = property.as_any().downcast_ref::<Color>() {
                assert_eq!(color.r, 255);
                assert_eq!(color.g, 0);
                assert_eq!(color.b, 0);
            } else {
                panic!("Expected a color property");
            }
        } else {
            panic!("Expected a property to be created");
        }
    }
}
