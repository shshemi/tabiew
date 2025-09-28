
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubDarkDimmed;

impl SixColorsTwoRowsStyler for GithubDarkDimmed {
    const BACKGROUND: Color = Color::from_u32(0x0022272e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0042474E);
    const FOREGROUND: Color = Color::from_u32(0x00adbac7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00545d68);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff938a),
        Color::from_u32(0x006bc46d),
        Color::from_u32(0x00daaa3f),
        Color::from_u32(0x006cb6ff),
        Color::from_u32(0x00dcbdfb),
        Color::from_u32(0x0056d4dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f47067),
        Color::from_u32(0x0057ab5a),
        Color::from_u32(0x00c69026),
        Color::from_u32(0x00539bf5),
        Color::from_u32(0x00b083f0),
        Color::from_u32(0x0039c5cf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282D34), Color::from_u32(0x002E333A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00337BD5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C44037);
}
