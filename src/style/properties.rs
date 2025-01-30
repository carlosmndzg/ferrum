use color::Color;

use crate::css::types::Declaration;

pub(crate) const AVAILABLE_PROPERTIES: [&str; 1] = ["color"];
pub(crate) const INHERITABLE_PROPERTIES: [&str; 1] = ["color"];

pub(crate) mod color;

pub(crate) struct PropertyFactory;

impl PropertyFactory {
    pub(crate) fn create_property(declaration: &Declaration) -> Option<Property> {
        match declaration.name.as_str() {
            "color" => Some(Property::Color(Color::maybe_new(&declaration.value)?)),
            _ => None,
        }
    }

    pub(crate) fn create_initial_property(name: &str) -> Property {
        match name {
            "color" => Property::Color(Color::default()),
            _ => panic!("Unknown property \"{}\"", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Property {
    Color(Color),
}

impl Property {
    pub(crate) fn name(&self) -> &str {
        match self {
            Property::Color(color) => color.name(),
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
            let Property::Color(Color { r, g, b }) = property;
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected a property to be created");
        }
    }
}
