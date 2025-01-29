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
            "white" => Color::wrap_color(255, 255, 255),
            "red" => Color::wrap_color(255, 0, 0),
            "green" => Color::wrap_color(0, 128, 0),
            "blue" => Color::wrap_color(0, 0, 255),
            "rebeccapurple" => Color::wrap_color(102, 51, 153),
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
