use background_color::BackgroundColor;
use color::Color;
use display::Display;
use font_size::FontSize;
use height::Height;
use margin::Margin;
use margin_bottom::MarginBottom;
use margin_left::MarginLeft;
use margin_right::MarginRight;
use margin_top::MarginTop;
use padding::Padding;
use width::Width;

use crate::css::types::Declaration;

pub(crate) const AVAILABLE_PROPERTIES: [&str; 12] = [
    "color",
    "display",
    "width",
    "height",
    "font-size",
    "background-color",
    "padding",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
];
pub(crate) const INHERITABLE_PROPERTIES: [&str; 2] = ["color", "font-size"];

pub(crate) mod background_color;
pub(crate) mod color;
pub(crate) mod display;
pub(crate) mod font_size;
pub(crate) mod height;
pub(crate) mod margin;
pub(crate) mod margin_bottom;
pub(crate) mod margin_left;
pub(crate) mod margin_right;
pub(crate) mod margin_top;
pub(crate) mod padding;
pub(crate) mod width;

pub(crate) struct PropertyFactory;

impl PropertyFactory {
    pub(crate) fn create_property(declaration: &Declaration) -> Option<Property> {
        match declaration.name.as_str() {
            "color" => Some(Property::Color(Color::maybe_new(&declaration.value)?)),
            "display" => Some(Property::Display(Display::maybe_new(&declaration.value)?)),
            "width" => Some(Property::Width(Width::maybe_new(&declaration.value)?)),
            "height" => Some(Property::Height(Height::maybe_new(&declaration.value)?)),
            "font-size" => Some(Property::FontSize(FontSize::maybe_new(&declaration.value)?)),
            "background-color" => Some(Property::BackgroundColor(BackgroundColor::maybe_new(&declaration.value)?)),
            "padding" => Some(Property::Padding(Padding::maybe_new(&declaration.value)?)),
            "margin" => Some(Property::Margin(Margin::maybe_new(&declaration.value)?)),
            "margin-top" => Some(Property::MarginTop(MarginTop::maybe_new(&declaration.value)?)),
            "margin-right" => Some(Property::MarginRight(MarginRight::maybe_new(&declaration.value)?)),
            "margin-bottom" => Some(Property::MarginBottom(MarginBottom::maybe_new(&declaration.value)?)),
            "margin-left" => Some(Property::MarginLeft(MarginLeft::maybe_new(&declaration.value)?)),
            _ => None,
        }
    }

    pub(crate) fn create_initial_property(name: &str) -> Property {
        match name {
            "color" => Property::Color(Color::default()),
            "display" => Property::Display(Display::default()),
            "width" => Property::Width(Width::default()),
            "height" => Property::Height(Height::default()),
            "font-size" => Property::FontSize(FontSize::default()),
            "background-color" => Property::BackgroundColor(BackgroundColor::default()),
            "padding" => Property::Padding(Padding::default()),
            "margin" => Property::Margin(Margin::default()),
            "margin-top" => Property::MarginTop(MarginTop::default()),
            "margin-right" => Property::MarginRight(MarginRight::default()),
            "margin-bottom" => Property::MarginBottom(MarginBottom::default()),
            "margin-left" => Property::MarginLeft(MarginLeft::default()),
            _ => panic!("Unknown property \"{}\"", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Property {
    Color(Color),
    Display(Display),
    Width(Width),
    Height(Height),
    FontSize(FontSize),
    BackgroundColor(BackgroundColor),
    Padding(Padding),
    Margin(Margin),
    MarginTop(MarginTop),
    MarginRight(MarginRight),
    MarginBottom(MarginBottom),
    MarginLeft(MarginLeft),
}

impl Property {
    pub(crate) fn name(&self) -> &str {
        match self {
            Property::Color(color) => color.name(),
            Property::Display(display) => display.name(),
            Property::Width(width) => width.name(),
            Property::Height(height) => height.name(),
            Property::FontSize(font_size) => font_size.name(),
            Property::BackgroundColor(background_color) => background_color.name(),
            Property::Padding(padding) => padding.name(),
            Property::Margin(margin) => margin.name(),
            Property::MarginTop(margin_top) => margin_top.name(),
            Property::MarginRight(margin_right) => margin_right.name(),
            Property::MarginBottom(margin_bottom) => margin_bottom.name(),
            Property::MarginLeft(margin_left) => margin_left.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{css::types::Value, style::types::Rgb};

    use super::*;

    #[test]
    fn test_create_property() {
        let declaration = Declaration {
            name: "color".to_string(),
            value: Value::Keyword("red".to_string()),
        };

        let property = PropertyFactory::create_property(&declaration);

        if let Some(property) = property {
            let Property::Color(Color {
                value: Rgb { r, g, b },
            }) = property
            else {
                panic!("Expected a property to be created");
            };

            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected a property to be created");
        }
    }
}
