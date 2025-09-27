
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NightOwlishLight;

impl SixColorsTwoRowsStyler for NightOwlishLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00403f53);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00011627);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f76e6e),
        Color::from_u32(0x0049d0c5),
        Color::from_u32(0x00dac26b),
        Color::from_u32(0x005ca7e4),
        Color::from_u32(0x00697098),
        Color::from_u32(0x0000c990),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d3423e),
        Color::from_u32(0x002aa298),
        Color::from_u32(0x00daaa01),
        Color::from_u32(0x004876d6),
        Color::from_u32(0x00403f53),
        Color::from_u32(0x0008916a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00201F33);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AA7A00);

    fn id(&self) -> &str {
        "night_owlish_light"
    }

    fn title(&self) -> &str {
        "NightOwlishLight"
    }
}
