use core::fmt;
use std::collections::HashMap;

use crate::{
    css::types::{Declaration, Value},
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

use super::properties::{display::Display, PropertyRegistry};

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

            if let Some(Property::$property_type(value)) = self.styles.get(&property_name) {
                return value;
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
        self.display().value() == &Value::Keyword("none".to_string())
    }

    pub(crate) fn is_block_level(&self) -> bool {
        self.display().value() == &Value::Keyword("block".to_string())
    }

    pub(crate) fn is_inline_level(&self) -> bool {
        self.display().value() == &Value::Keyword("inline".to_string())
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
    pub(crate) fn add(&mut self, property: Property) {
        self.properties
            .insert(property.name().to_string(), property);
    }

    pub(crate) fn has(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }

    pub(crate) fn apply(
        &mut self,
        declarations: &[Declaration],
        property_registry: &PropertyRegistry,
    ) {
        for declaration in declarations {
            let property_name = &declaration.name;
            let property_value = &declaration.value;

            for property in property_registry.create(property_name, property_value) {
                self.add(property);
            }
        }
    }
}
