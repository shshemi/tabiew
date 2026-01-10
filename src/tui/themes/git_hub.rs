use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Github;

impl SixColorsTwoRowsStyler for Github {
    const BACKGROUND: Color = Color::from_u32(0x00f4f4f4);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x003e3e3e);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003e3e3e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de0000),
        Color::from_u32(0x0087d5a2),
        Color::from_u32(0x00f1d007),
        Color::from_u32(0x002e6cba),
        Color::from_u32(0x00ffa29f),
        Color::from_u32(0x001cfafe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00970b16),
        Color::from_u32(0x0007962a),
        Color::from_u32(0x00f8eec7),
        Color::from_u32(0x00003e8a),
        Color::from_u32(0x00e94691),
        Color::from_u32(0x0089d1ec),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FAFAFA), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x001F1F1F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C8BE97);
}
