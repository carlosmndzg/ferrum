use font_kit::{
    family_name::FamilyName,
    font::Font,
    properties::{Properties, Weight},
    source::SystemSource,
};
use std::collections::HashMap;

pub struct FontsContext {
    pub fonts: HashMap<u32, Font>,
}

impl FontsContext {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn add_font_if_not_exists(&mut self, font_weight: u32) -> &Font {
        self.fonts.entry(font_weight).or_insert_with(|| {
            SystemSource::new()
                .select_best_match(
                    &[FamilyName::SansSerif],
                    &Properties {
                        weight: Weight(font_weight as f32),
                        ..Default::default()
                    },
                )
                .unwrap()
                .load()
                .unwrap()
        })
    }

    pub fn get_font(&self, font_weight: u32) -> &Font {
        self.fonts.get(&font_weight).expect("Font not found")
    }
}
