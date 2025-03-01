use std::collections::HashMap;

use background_color::BackgroundColor;
use border_color::BorderColor;
use border_style::BorderStyle;
use border_width::BorderWidth;
use color::Color;
use display::Display;
use font_size::FontSize;
use font_weight::FontWeight;
use height::Height;
use line_height::LineHeight;
use margin::Margin;
use margin_bottom::MarginBottom;
use margin_left::MarginLeft;
use margin_right::MarginRight;
use margin_top::MarginTop;
use padding::Padding;
use padding_bottom::PaddingBottom;
use padding_left::PaddingLeft;
use padding_right::PaddingRight;
use padding_top::PaddingTop;
use text_align::TextAlign;
use width::Width;

use crate::css::types::Value;

pub(crate) mod background_color;
pub(crate) mod border_color;
pub(crate) mod border_style;
pub(crate) mod border_width;
pub(crate) mod color;
pub(crate) mod display;
pub(crate) mod font_size;
pub(crate) mod font_weight;
pub(crate) mod height;
pub(crate) mod line_height;
pub(crate) mod margin;
pub(crate) mod margin_bottom;
pub(crate) mod margin_left;
pub(crate) mod margin_right;
pub(crate) mod margin_top;
pub(crate) mod padding;
pub(crate) mod padding_bottom;
pub(crate) mod padding_left;
pub(crate) mod padding_right;
pub(crate) mod padding_top;
pub(crate) mod text_align;
pub(crate) mod width;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Property {
    BackgroundColor(BackgroundColor),
    BorderColor(BorderColor),
    BorderStyle(BorderStyle),
    BorderWidth(BorderWidth),
    Color(Color),
    Display(Display),
    FontSize(FontSize),
    FontWeight(FontWeight),
    Height(Height),
    LineHeight(LineHeight),
    Margin(Margin),
    MarginTop(MarginTop),
    MarginRight(MarginRight),
    MarginBottom(MarginBottom),
    MarginLeft(MarginLeft),
    Padding(Padding),
    PaddingTop(PaddingTop),
    PaddingRight(PaddingRight),
    PaddingBottom(PaddingBottom),
    PaddingLeft(PaddingLeft),
    TextAlign(TextAlign),
    Width(Width),
}

impl Property {
    pub(crate) fn name(&self) -> &str {
        match self {
            Property::BackgroundColor(property) => property.name(),
            Property::BorderColor(property) => property.name(),
            Property::BorderStyle(property) => property.name(),
            Property::BorderWidth(property) => property.name(),
            Property::Color(property) => property.name(),
            Property::Display(property) => property.name(),
            Property::FontSize(property) => property.name(),
            Property::FontWeight(property) => property.name(),
            Property::Height(property) => property.name(),
            Property::LineHeight(property) => property.name(),
            Property::Margin(property) => property.name(),
            Property::MarginTop(property) => property.name(),
            Property::MarginRight(property) => property.name(),
            Property::MarginBottom(property) => property.name(),
            Property::MarginLeft(property) => property.name(),
            Property::Padding(property) => property.name(),
            Property::PaddingTop(property) => property.name(),
            Property::PaddingRight(property) => property.name(),
            Property::PaddingBottom(property) => property.name(),
            Property::PaddingLeft(property) => property.name(),
            Property::TextAlign(property) => property.name(),
            Property::Width(property) => property.name(),
        }
    }
}

/// A registry for CSS properties.
/// This struct is responsible for managing the properties that can be used in CSS
pub(crate) struct PropertyRegistry {
    properties: HashMap<&'static str, Box<dyn CssProperty>>,
    inheritable_properties: Vec<&'static str>,
    available_properties: Vec<&'static str>,
}

impl PropertyRegistry {
    /// Creates a new `PropertyRegistry` instance with the default properties registered.
    pub(crate) fn new() -> Self {
        let mut property_builder = PropertyRegistry {
            properties: HashMap::new(),
            inheritable_properties: Vec::new(),
            available_properties: Vec::new(),
        };

        property_builder.register(Box::new(BackgroundColor::new()));
        property_builder.register(Box::new(BorderStyle::new()));
        property_builder.register(Box::new(BorderWidth::new()));
        property_builder.register(Box::new(BorderColor::new()));
        property_builder.register(Box::new(Color::new()));
        property_builder.register(Box::new(Display::new()));
        property_builder.register(Box::new(FontSize::new()));
        property_builder.register(Box::new(FontWeight::new()));
        property_builder.register(Box::new(Height::new()));
        property_builder.register(Box::new(LineHeight::new()));
        property_builder.register(Box::new(Margin::new()));
        property_builder.register(Box::new(MarginTop::new()));
        property_builder.register(Box::new(MarginRight::new()));
        property_builder.register(Box::new(MarginBottom::new()));
        property_builder.register(Box::new(MarginLeft::new()));
        property_builder.register(Box::new(Padding::new()));
        property_builder.register(Box::new(PaddingTop::new()));
        property_builder.register(Box::new(PaddingRight::new()));
        property_builder.register(Box::new(PaddingBottom::new()));
        property_builder.register(Box::new(PaddingLeft::new()));
        property_builder.register(Box::new(TextAlign::new()));
        property_builder.register(Box::new(Width::new()));

        property_builder
    }

    fn register(&mut self, property: Box<dyn CssProperty>) {
        if property.is_inheritable() {
            self.inheritable_properties.push(property.name());
        }

        if !property.is_shorthand() {
            self.available_properties.push(property.name());
        }

        self.properties.insert(property.name(), property);
    }

    /// Creates properties based on the provided name and values.
    /// Due to the nature of CSS, a property can set multiple properties (shorthand properties).
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name of the CSS property.
    /// * `value` - A slice of `Value` instances representing the CSS property values.
    ///
    /// # Returns
    ///
    /// A vector of `Property` instances based on the provided name and values.
    pub(crate) fn create(&self, name: &str, value: &[Value]) -> impl IntoIterator<Item = Property> {
        let property = self.properties.get(name);

        if let Some(property) = property {
            return property.maybe_new(value);
        }

        Vec::new()
    }

    /// Returns the initial value of the CSS property based on the provided name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name of the CSS property.
    ///
    /// # Returns
    ///
    /// An owned iterator of `Property` instances representing the initial value of the CSS property.
    pub(crate) fn initial_value(&self, name: &str) -> impl IntoIterator<Item = Property> {
        let property = self.properties.get(name);

        if let Some(property) = property {
            return property.initial_value();
        }

        Vec::new()
    }

    /// Returns the list of inheritable properties.
    pub(crate) fn inheritable_properties(&self) -> &[&'static str] {
        &self.inheritable_properties
    }

    /// Returns the list of available properties (shorthand properties not included).
    pub(crate) fn available_properties(&self) -> &[&'static str] {
        &self.available_properties
    }
}

/// A trait representing a CSS property.
trait CssProperty {
    /// Returns the name of the CSS property.
    fn name(&self) -> &'static str;

    /// Returns whether the CSS property is inheritable or not.
    fn is_inheritable(&self) -> bool;

    /// Returns the initial value of the CSS property, if any (short hand properties do not provide initial value).
    fn initial_value(&self) -> Vec<Property>;

    /// Returns whether the CSS property is a shorthand property or not.
    fn is_shorthand(&self) -> bool;

    /// Attempts to create a new `Property` from the given slice of `Value`s.
    ///
    /// # Arguments
    ///
    /// * `value` - A slice of `Value` instances representing the CSS property values.
    ///
    /// # Returns
    ///
    /// * `Some(Property)` if the values can be successfully converted into a `Property`.
    /// * `None` if the values cannot be converted into a `Property`.
    ///
    /// This method allows for the creation of a `Property` based on the provided values,
    /// returning `None` if the conversion is not possible.
    fn maybe_new(&self, value: &[Value]) -> Vec<Property>;
}
