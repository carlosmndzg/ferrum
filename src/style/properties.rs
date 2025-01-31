use color::Color;
use display::Display;

use crate::css::types::Declaration;

pub(crate) const AVAILABLE_PROPERTIES: [&str; 2] = ["color", "display"];
pub(crate) const INHERITABLE_PROPERTIES: [&str; 1] = ["color"];

pub(crate) mod color;
pub(crate) mod display;

pub(crate) struct PropertyFactory;

impl PropertyFactory {
    pub(crate) fn create_property(declaration: &Declaration) -> Option<Property> {
        match declaration.name.as_str() {
            "color" => Some(Property::Color(Color::maybe_new(&declaration.value)?)),
            "display" => Some(Property::Display(Display::maybe_new(&declaration.value)?)),
            _ => None,
        }
    }

    pub(crate) fn create_initial_property(name: &str) -> Property {
        match name {
            "color" => Property::Color(Color::default()),
            "display" => Property::Display(Display::default()),
            _ => panic!("Unknown property \"{}\"", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Property {
    Color(Color),
    Display(Display),
}

impl Property {
    pub(crate) fn name(&self) -> &str {
        match self {
            Property::Color(color) => color.name(),
            Property::Display(display) => display.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::css::types::Value;

    use super::*;

    #[test]
    fn test_create_property() {
        let declaration = Declaration {
            name: "color".to_string(),
            value: Value::Keyword("red".to_string()),
        };

        let property = PropertyFactory::create_property(&declaration);

        if let Some(property) = property {
            let Property::Color(Color { r, g, b }) = property else {
                panic!("Expected a color property");
            };
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected a property to be created");
        }
    }
}
