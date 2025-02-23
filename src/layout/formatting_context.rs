use std::{collections::HashMap, mem::take};

use font_kit::{
    family_name::FamilyName,
    font::Font,
    properties::{Properties, Weight},
    source::SystemSource,
};

use crate::{
    style::{properties::text_align::TextAlign, types::Rgb},
    NodeType,
};

use super::{
    box_types::{block::Block, inline::Inline, word::Word as WordBox},
    types::{BoxType, LayoutNode},
};

#[derive(Debug, PartialEq)]
pub(crate) enum FormattingContext {
    Block,
    Inline,
}

impl FormattingContext {
    pub(crate) fn handle(
        &self,
        node: &mut LayoutNode,
        text_alignment: &TextAlign,
        desired_height: Option<f32>,
    ) {
        match self {
            FormattingContext::Block => self.handle_block(node, desired_height),
            FormattingContext::Inline => self.handle_inline(node, text_alignment, desired_height),
        }
    }

    fn handle_block(&self, node: &mut LayoutNode, desired_height: Option<f32>) {
        for child in &mut node.children {
            let child_desired_height = child.compute_desired_height(desired_height);

            child.compute_layout(&node.box_dimensions, child_desired_height);

            node.box_dimensions.content.height += child.box_dimensions.margin.top
                + child.box_dimensions.border.top
                + child.box_dimensions.padding.top
                + child.box_dimensions.content.height
                + child.box_dimensions.padding.bottom
                + child.box_dimensions.border.bottom
                + child.box_dimensions.margin.bottom;
        }

        if let Some(height) = desired_height {
            node.box_dimensions.content.height = height;
        }
    }

    fn handle_inline(
        &self,
        node: &mut LayoutNode,
        text_alignment: &TextAlign,
        desired_height: Option<f32>,
    ) {
        if node.children.is_empty()
            || matches!(node.children[0].box_type, BoxType::Block(Block { node, .. }, ..) if node.is_empty_text_node())
        {
            return;
        }

        let containing_block_width = node.box_dimensions.content.width;
        let containing_block_x = node.box_dimensions.content.x;
        let containing_block_y = node.box_dimensions.content.y;

        let mut current_line = LayoutNode::create_line_box();
        current_line.box_dimensions.content.width = 0.0;
        current_line.box_dimensions.content.x = containing_block_x;

        let words = WordBuilder::generate_vector_from_layout_nodes(take(&mut node.children));

        let mut i = 0;

        while i < words.len() {
            let word = &words[i];

            if (current_line.children.is_empty() && !word.text.trim().is_empty())
                || (current_line.box_dimensions.content.width + word.width
                    <= containing_block_width
                    && !current_line.children.is_empty())
            {
                let height = word.font_size;
                let width = word.width;
                let mut word = LayoutNode::create_word_box(
                    word.text.clone(),
                    word.font_size,
                    word.line_height,
                    word.font_weight,
                    word.color.clone(),
                );

                word.box_dimensions.content.width = width;
                word.box_dimensions.content.height = height;
                word.box_dimensions.content.x =
                    current_line.box_dimensions.content.width + containing_block_x;

                current_line.box_dimensions.content.width += width;
                current_line.children.push(word);
                i += 1;
            } else if word.text.trim().is_empty() {
                i += 1;
            } else {
                node.children.push(current_line);
                current_line = LayoutNode::create_line_box();
                current_line.box_dimensions.content.width = 0.0;
                current_line.box_dimensions.content.x = containing_block_x;
            }
        }

        if !current_line.children.is_empty() {
            node.children.push(current_line);
        }

        // We clean space at the end of the lines if it exists
        for line in &mut node.children {
            let last_word = line.children.last().expect("Expected last word");
            let BoxType::Word(WordBox { text, .. }) = &last_word.box_type else {
                panic!("Expected word box type");
            };

            if text.trim().is_empty() {
                line.box_dimensions.content.width -= last_word.box_dimensions.content.width;
                line.children.pop();
            }
        }

        text_alignment.apply(node);

        let mut acc_height = 0.0;

        for line in &mut node.children {
            let mut max = 0.0;
            let mut max_font_size = 0.0;

            for word in &line.children {
                let BoxType::Word(WordBox { line_height, .. }) = &word.box_type else {
                    panic!("Expected word box type");
                };

                if word.box_dimensions.content.height > max {
                    max = word.box_dimensions.content.height * line_height;
                }

                if word.box_dimensions.content.height > max_font_size {
                    max_font_size = word.box_dimensions.content.height;
                }
            }

            line.box_dimensions.content.height = max;
            line.box_dimensions.content.y = containing_block_y + acc_height;
            acc_height += max;

            let initial_y = line.box_dimensions.content.y + line.box_dimensions.content.height
                - ((line.box_dimensions.content.height * 1.1 - max_font_size) / 2.);

            for word in &mut line.children {
                word.box_dimensions.content.y = initial_y;
            }
        }

        if let Some(height) = desired_height {
            node.box_dimensions.content.height = height;
        } else {
            node.box_dimensions.content.height = acc_height;
        }
    }
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
            let BoxType::Inline(Inline {
                node: styled_node, ..
            }) = node.box_type
            else {
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
