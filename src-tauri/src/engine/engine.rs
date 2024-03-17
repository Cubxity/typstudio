use crate::engine::{FontSearcher, FontSlot};
use comemo::Prehashed;
use typst::text::FontBook;
use typst::Library;

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
            library: Prehashed::new(Library::default()),
            fontbook: Prehashed::new(searcher.book),
            fonts: searcher.fonts,
        }
    }
}
