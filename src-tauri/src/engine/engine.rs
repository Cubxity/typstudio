use crate::engine::{FontSearcher, FontSlot};
use comemo::Prehashed;
use typst::eval::Library;
use typst::font::FontBook;

pub struct TypstEngine {
    pub library: Prehashed<Library>,
    pub fontbook: Prehashed<FontBook>,
    pub fonts: Vec<FontSlot>,
}

impl TypstEngine {
    pub fn new() -> Self {
        let mut searcher = FontSearcher::new();
        searcher.search(&[]);

        Self {
            library: Prehashed::new(typst_library::build()),
            fontbook: Prehashed::new(searcher.book),
            fonts: searcher.fonts,
        }
    }
}
