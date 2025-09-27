
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Twilight;

impl SixColorsTwoRowsStyler for Twilight {
    const BACKGROUND: Color = Color::from_u32(0x00141414);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343434);
    const FOREGROUND: Color = Color::from_u32(0x00ffffd4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00141414);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de7c4c),
        Color::from_u32(0x00ccd88c),
        Color::from_u32(0x00e2c47e),
        Color::from_u32(0x005a5e62),
        Color::from_u32(0x00d0dc8e),
        Color::from_u32(0x008a989b),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c06d44),
        Color::from_u32(0x00afb97a),
        Color::from_u32(0x00c2a86c),
        Color::from_u32(0x0044474a),
        Color::from_u32(0x00b4be7c),
        Color::from_u32(0x00778385),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1A1A), Color::from_u32(0x00202020)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0092783C);

    fn id(&self) -> &str {
        "twilight"
    }

    fn title(&self) -> &str {
        "Twilight"
    }
}
