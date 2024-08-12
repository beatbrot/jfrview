use egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;

pub fn load_fonts() -> FontDefinitions {
    let mut font_data: BTreeMap<String, FontData> = BTreeMap::new();
    let mut families: BTreeMap<FontFamily, Vec<String>> = BTreeMap::new();

    font_data.insert(
        "Ubuntu-Light".to_owned(),
        FontData::from_static(include_bytes!("../../fonts/Ubuntu-Light.ttf")),
    );

    families.insert(FontFamily::Proportional, vec!["Ubuntu-Light".to_owned()]);
    families.insert(FontFamily::Monospace, vec![]);

    FontDefinitions {
        font_data,
        families,
    }
}
