
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubLightColorblind;

impl SixColorsTwoRowsStyler for GithubLightColorblind {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0024292f);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0024292f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x008a4600),
        Color::from_u32(0x000969da),
        Color::from_u32(0x00633c01),
        Color::from_u32(0x00218bff),
        Color::from_u32(0x00a475f9),
        Color::from_u32(0x003192aa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b35900),
        Color::from_u32(0x000550ae),
        Color::from_u32(0x004d2d00),
        Color::from_u32(0x000969da),
        Color::from_u32(0x008250df),
        Color::from_u32(0x001b7c83),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x000049BA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00832900);
}
