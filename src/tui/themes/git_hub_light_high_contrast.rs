use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubLightHighContrast;

impl SixColorsTwoRowsStyler for GithubLightHighContrast {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x000e1116);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000e1116);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0086061d),
        Color::from_u32(0x00055d20),
        Color::from_u32(0x004e2c00),
        Color::from_u32(0x001168e3),
        Color::from_u32(0x00844ae7),
        Color::from_u32(0x003192aa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a0111f),
        Color::from_u32(0x00024c1a),
        Color::from_u32(0x003f2200),
        Color::from_u32(0x000349b4),
        Color::from_u32(0x00622cbc),
        Color::from_u32(0x001b7c83),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00002994);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00700000);
}
