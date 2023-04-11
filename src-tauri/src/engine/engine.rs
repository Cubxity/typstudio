use comemo::Prehashed;
use typst::eval::Library;
use typst::font::{Font, FontBook};
use typst::util::Buffer;

pub struct TypstEngine {
    pub library: Prehashed<Library>,
    pub fontbook: Prehashed<FontBook>,
    pub fonts: Vec<Font>,
}

impl TypstEngine {
    pub fn new() -> Self {
        // https://github.com/typst/typst/blob/085282c138899dd5aaa06bc6ae7bd2f79d75d7e1/cli/src/main.rs#L695
        const EMBEDDED_FONTS: [&[u8]; 10] = [
            include_bytes!("../../assets/fonts/LinLibertine_R.ttf"),
            include_bytes!("../../assets/fonts/LinLibertine_RB.ttf"),
            include_bytes!("../../assets/fonts/LinLibertine_RBI.ttf"),
            include_bytes!("../../assets/fonts/LinLibertine_RI.ttf"),
            include_bytes!("../../assets/fonts/NewCMMath-Book.otf"),
            include_bytes!("../../assets/fonts/NewCMMath-Regular.otf"),
            include_bytes!("../../assets/fonts/DejaVuSansMono.ttf"),
            include_bytes!("../../assets/fonts/DejaVuSansMono-Bold.ttf"),
            include_bytes!("../../assets/fonts/DejaVuSansMono-Oblique.ttf"),
            include_bytes!("../../assets/fonts/DejaVuSansMono-BoldOblique.ttf"),
        ];

        let mut fontbook = FontBook::new();
        let mut fonts = vec![];

        for file in EMBEDDED_FONTS {
            for font in Font::iter(Buffer::from_static(file)) {
                fontbook.push(font.info().clone());
                fonts.push(font);
            }
        }

        Self {
            library: Prehashed::new(typst_library::build()),
            fontbook: Prehashed::new(fontbook),
            fonts,
        }
    }
}
