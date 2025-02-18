use core::{fmt, panic};
use std::collections::HashMap;

use font_kit::{
    family_name::FamilyName,
    font::Font,
    properties::{Properties, Weight},
    source::SystemSource,
};

use crate::{
    style::types::{Rgb, StyledNode},
    NodeType,
};

#[derive(Default)]
pub(crate) struct LayoutNode<'a> {
    pub(crate) box_dimensions: BoxDimensions,
    pub(crate) box_type: BoxType<'a>,
    pub(crate) children: Vec<LayoutNode<'a>>,
}

impl fmt::Debug for LayoutNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayoutNode")
            .field("box_type", &self.box_type.box_representation())
            .field("box_dimensions", &self.box_dimensions)
            .field("children", &self.children)
            .finish()
    }
}

impl<'a> LayoutNode<'a> {
    pub(crate) fn create_line_box() -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions::default(),
            box_type: BoxType::Line,
            children: Vec::new(),
        }
    }

    pub(crate) fn create_word_box(
        text: String,
        font_size: f32,
        line_height: f32,
        font_weight: u32,
        color: Rgb,
    ) -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions::default(),
            box_type: BoxType::Word {
                text,
                font_size,
                line_height,
                font_weight,
                color,
            },
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct BoxDimensions {
    pub(crate) content: Rectangle,
    pub(crate) padding: EdgeSizes,
    pub(crate) border: EdgeSizes,
    pub(crate) margin: EdgeSizes,
}

impl BoxDimensions {
    pub(crate) fn border_box(&self) -> Rectangle {
        let mut border_box = self.content.clone();

        border_box.x -= self.padding.left + self.border.left;
        border_box.y -= self.padding.top + self.border.top;
        border_box.width +=
            self.padding.left + self.padding.right + self.border.left + self.border.right;
        border_box.height +=
            self.padding.top + self.padding.bottom + self.border.top + self.border.bottom;

        border_box
    }

    pub(crate) fn padding_box(&self) -> Rectangle {
        let mut padding_box = self.content.clone();

        padding_box.x -= self.padding.left;
        padding_box.y -= self.padding.top;
        padding_box.width += self.padding.left + self.padding.right;
        padding_box.height += self.padding.top + self.padding.bottom;

        padding_box
    }
}

#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub(crate) struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct EdgeSizes {
    pub(crate) top: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
    pub(crate) right: f32,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) enum BoxType<'a> {
    Block(&'a StyledNode<'a>, FormattingContext),
    Inline(&'a StyledNode<'a>),
    #[default]
    Anonymous,
    Line,
    Word {
        text: String,
        font_size: f32,
        line_height: f32,
        font_weight: u32,
        color: Rgb,
    },
}

impl BoxType<'_> {
    pub(crate) fn box_representation(&self) -> String {
        match self {
            BoxType::Block(node, ..) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Block | {}", e.tag_name);
                }

                panic!("Node type not supported");
            }
            BoxType::Inline(node) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Inline element | {}", e.tag_name);
                }

                if let NodeType::Text(t) = node_type {
                    return format!("Inline text | {}", t.text);
                }

                panic!("Node type not supported");
            }
            BoxType::Anonymous => "Anonymous".to_string(),
            BoxType::Line => "Line".to_string(),
            BoxType::Word { text, .. } => format!("Word | {}", text),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum FormattingContext {
    Block,
    Inline,
}

pub(crate) struct WordBuilder;

#[derive(Debug)]
pub(crate) struct Word {
    pub(crate) text: String,
    pub(crate) width: f32,
    pub(crate) line_height: f32,
    pub(crate) font_size: f32,
    pub(crate) font_weight: u32,
    pub(crate) color: Rgb,
}

impl WordBuilder {
    pub(crate) fn generate_vector_from_layout_nodes(layout_nodes: Vec<LayoutNode>) -> Vec<Word> {
        let mut words = Vec::new();

        for node in layout_nodes {
            let BoxType::Inline(styled_node) = node.box_type else {
                panic!("Expected inline node");
            };

            let node_type = &styled_node.node.node_type;

            if let NodeType::Element(_) = node_type {
                words.extend(WordBuilder::generate_vector_from_layout_nodes(
                    node.children,
                ));
            } else if let NodeType::Text(t) = node_type {
                let line_height = styled_node.line_height().value();
                let font_size = styled_node.font_size().value();
                let color = &styled_node.color().value();
                let font_weight = styled_node.font_weight().value();
                let text = &t.text;

                let mut word = String::new();

                for c in text.chars() {
                    if c.is_whitespace() {
                        if !word.is_empty() {
                            words.push(Word {
                                text: word.clone(),
                                width: 0.0,
                                line_height,
                                font_size,
                                font_weight,
                                color: color.clone(),
                            });

                            word.clear();
                        }

                        words.push(Word {
                            text: c.to_string(),
                            width: 0.0,
                            line_height,
                            font_size,
                            font_weight,
                            color: color.clone(),
                        });
                    } else {
                        word.push(c);
                    }
                }

                if !word.is_empty() {
                    words.push(Word {
                        text: word,
                        width: 0.0,
                        line_height,
                        font_size,
                        font_weight,
                        color: color.clone(),
                    });
                }
            }
        }

        let mut i = 0;

        while i < words.len() {
            if words[i].text.trim().is_empty() {
                let mut j = i + 1;

                while j < words.len() && words[j].text.trim().is_empty() {
                    j += 1;
                }

                words[i].text = " ".to_string();
                words.drain(i + 1..j);
            }

            i += 1;
        }

        if let Some(first_word) = words.first() {
            if first_word.text.trim().is_empty() {
                words.remove(0);
            }
        }

        if let Some(last_word) = words.last() {
            if last_word.text.trim().is_empty() {
                words.pop();
            }
        }

        let mut font_map = HashMap::new();

        for word in &mut words {
            let font = font_map.entry(word.font_weight).or_insert_with(|| {
                SystemSource::new()
                    .select_best_match(
                        &[FamilyName::SansSerif],
                        &Properties {
                            weight: Weight(word.font_weight as f32),
                            ..Default::default()
                        },
                    )
                    .unwrap()
                    .load()
                    .unwrap()
            });

            word.width = WordBuilder::measure_word_width(&word.text, font, word.font_size);
        }

        words
    }

    fn measure_word_width(text: &str, font: &Font, size: f32) -> f32 {
        let mut total_width = 0.0;

        let glyphs: Vec<u32> = text
            .chars()
            .filter_map(|c| font.glyph_for_char(c))
            .collect();

        for glyph in glyphs {
            if let Ok(advance) = font.advance(glyph) {
                total_width += advance.x() * (size / font.metrics().units_per_em as f32);
            }
        }

        total_width * 0.9
    }
}
