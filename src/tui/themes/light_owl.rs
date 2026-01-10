use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LightOwl;

impl SixColorsTwoRowsStyler for LightOwl {
    const BACKGROUND: Color = Color::from_u32(0x00fbfbfb);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00403f53);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00403f53);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de3d3b),
        Color::from_u32(0x0008916a),
        Color::from_u32(0x00daaa01),
        Color::from_u32(0x00288ed7),
        Color::from_u32(0x00d6438a),
        Color::from_u32(0x002aa298),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00de3d3b),
        Color::from_u32(0x0008916a),
        Color::from_u32(0x00e0af02),
        Color::from_u32(0x00288ed7),
        Color::from_u32(0x00d6438a),
        Color::from_u32(0x002aa298),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00201F33);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B07F00);
}
