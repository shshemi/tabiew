
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AtomOneLight;

impl SixColorsTwoRowsStyler for AtomOneLight {
    const BACKGROUND: Color = Color::from_u32(0x00f9f9f9);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x002a2c33);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de3e35),
        Color::from_u32(0x003f953a),
        Color::from_u32(0x00d2b67c),
        Color::from_u32(0x002f5af3),
        Color::from_u32(0x00a00095),
        Color::from_u32(0x003f953a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00de3e35),
        Color::from_u32(0x003f953a),
        Color::from_u32(0x00d2b67c),
        Color::from_u32(0x002f5af3),
        Color::from_u32(0x00950095),
        Color::from_u32(0x003f953a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AE0E05);

    fn id(&self) -> &str {
        "atom_one_light"
    }

    fn title(&self) -> &str {
        "AtomOneLight"
    }
}
