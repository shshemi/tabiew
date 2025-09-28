
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SpacegrayEighties;

impl SixColorsTwoRowsStyler for SpacegrayEighties {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00bdbaae);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0015171c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6973),
        Color::from_u32(0x0093d493),
        Color::from_u32(0x00ffd256),
        Color::from_u32(0x004d84d1),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x0083e9e4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec5f67),
        Color::from_u32(0x0081a764),
        Color::from_u32(0x00fec254),
        Color::from_u32(0x005486c0),
        Color::from_u32(0x00bf83c1),
        Color::from_u32(0x0057c2c1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE9224);
}
