use crate::css::types::{Unit, Value};

pub(crate) struct Validations;

impl Validations {
    pub(crate) fn length(value: &Value) -> bool {
        if let Value::Dimension(value, unit) = value {
            if *value == 0. {
                return true;
            }

            return matches!(unit, Unit::Px);
        }

        false
    }

    pub(crate) fn percentage(value: &Value) -> bool {
        matches!(value, Value::Percentage(_))
    }

    pub(crate) fn keyword(value: &Value, keywords: &[&'static str]) -> bool {
        if let Value::Keyword(keyword) = value {
            return keywords.contains(&keyword.as_str());
        }

        false
    }

    pub(crate) fn wide_keyword(value: &Value) -> bool {
        Validations::keyword(value, &["inherit", "initial", "unset"])
    }

    pub(crate) fn margin_width(value: &Value) -> bool {
        Validations::length(value)
            || Validations::percentage(value)
            || Validations::keyword(value, &["auto"])
    }

    pub(crate) fn padding_width(value: &Value) -> bool {
        Validations::length(value) || Validations::percentage(value)
    }

    pub(crate) fn border_style(value: &Value) -> bool {
        Validations::keyword(value, &["none", "hidden", "solid"])
    }

    pub(crate) fn border_width(value: &Value) -> bool {
        Validations::length(value) || Validations::keyword(value, &["thin", "medium", "thick"])
    }

    pub(crate) fn numbers(value: &Value, numbers: &[i32]) -> bool {
        if let Value::Dimension(number, Unit::None) = value {
            if number.fract() == 0.0 {
                return numbers.contains(&(*number as i32));
            }
        }

        false
    }

    pub(crate) fn font_weight(value: &Value) -> bool {
        Validations::keyword(value, &["normal", "bold"])
            || Validations::numbers(value, &[100, 200, 300, 400, 500, 600, 700, 800, 900])
    }

    pub(crate) fn number(value: &Value) -> bool {
        matches!(value, Value::Dimension(_, Unit::None))
    }

    pub(crate) fn color(value: &Value) -> bool {
        if matches!(value, Value::Rgb(_))
            || Validations::keyword(
                value,
                &[
                    "black",
                    "silver",
                    "gray",
                    "white",
                    "maroon",
                    "red",
                    "purple",
                    "fuchsia",
                    "green",
                    "lime",
                    "olive",
                    "yellow",
                    "navy",
                    "blue",
                    "teal",
                    "aqua",
                    "orange",
                    "aliceblue",
                    "antiquewhite",
                    "aquamarine",
                    "azure",
                    "beige",
                    "bisque",
                    "blanchedalmond",
                    "blueviolet",
                    "brown",
                    "burlywood",
                    "cadetblue",
                    "chartreuse",
                    "chocolate",
                    "coral",
                    "cornflowerblue",
                    "cornsilk",
                    "crimson",
                    "cyan",
                    "darkblue",
                    "darkcyan",
                    "darkgoldenrod",
                    "darkgray",
                    "darkgreen",
                    "darkgrey",
                    "darkkhaki",
                    "darkmagenta",
                    "darkolivegreen",
                    "darkorange",
                    "darkorchid",
                    "darkred",
                    "darksalmon",
                    "darkseagreen",
                    "darkslateblue",
                    "darkslategray",
                    "darkslategrey",
                    "darkturquoise",
                    "darkviolet",
                    "deeppink",
                    "deepskyblue",
                    "dimgray",
                    "dimgrey",
                    "dodgerblue",
                    "firebrick",
                    "floralwhite",
                    "forestgreen",
                    "gainsboro",
                    "ghostwhite",
                    "gold",
                    "goldenrod",
                    "greenyellow",
                    "grey",
                    "honeydew",
                    "hotpink",
                    "indianred",
                    "indigo",
                    "ivory",
                    "khaki",
                    "lavender",
                    "lavenderblush",
                    "lawngreen",
                    "lemonchiffon",
                    "lightblue",
                    "lightcoral",
                    "lightcyan",
                    "lightgoldenrodyellow",
                    "lightgray",
                    "lightgreen",
                    "lightgrey",
                    "lightpink",
                    "lightsalmon",
                    "lightseagreen",
                    "lightskyblue",
                    "lightslategray",
                    "lightslategrey",
                    "lightsteelblue",
                    "lightyellow",
                    "limegreen",
                    "linen",
                    "magenta",
                    "mediumaquamarine",
                    "mediumblue",
                    "mediumorchid",
                    "mediumpurple",
                    "mediumseagreen",
                    "mediumslateblue",
                    "mediumspringgreen",
                    "mediumturquoise",
                    "mediumvioletred",
                    "midnightblue",
                    "mintcream",
                    "mistyrose",
                    "moccasin",
                    "navajowhite",
                    "oldlace",
                    "olivedrab",
                    "orangered",
                    "orchid",
                    "palegoldenrod",
                    "palegreen",
                    "paleturquoise",
                    "palevioletred",
                    "papayawhip",
                    "peachpuff",
                    "peru",
                    "pink",
                    "plum",
                    "powderblue",
                    "rebeccapurple",
                    "rosybrown",
                    "royalblue",
                    "saddlebrown",
                    "salmon",
                    "sandybrown",
                    "seagreen",
                    "seashell",
                    "sienna",
                    "skyblue",
                    "slateblue",
                    "slategray",
                    "slategrey",
                    "snow",
                    "springgreen",
                    "steelblue",
                    "tan",
                    "thistle",
                    "tomato",
                    "turquoise",
                    "violet",
                    "wheat",
                    "whitesmoke",
                    "yellowgreen",
                    "transparent",
                ],
            )
        {
            return true;
        }

        false
    }
}
