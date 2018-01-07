use rusttype;
use std::collections::HashMap;

struct FontManager <'a> {
    font_data : HashMap<FontId, rusttype::FontCollection<'a>>
}

type FontId = String;

impl <'a> FontManager <'a> {
    pub fn new() -> FontManager <'a> {
        FontManager {
            font_data: HashMap::new(),
        }
    }

    pub fn register_font()
}