
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TomorrowNightBlue;

impl SixColorsTwoRowsStyler for TomorrowNightBlue {
    const BACKGROUND: Color = Color::from_u32(0x00002451);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00204471);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9da4),
        Color::from_u32(0x00d1f1a9),
        Color::from_u32(0x00ffeead),
        Color::from_u32(0x00bbdaff),
        Color::from_u32(0x00ebbbff),
        Color::from_u32(0x0099ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9da4),
        Color::from_u32(0x00d1f1a9),
        Color::from_u32(0x00ffeead),
        Color::from_u32(0x00bbdaff),
        Color::from_u32(0x00ebbbff),
        Color::from_u32(0x0099ffff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00062A57), Color::from_u32(0x000C305D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF6D74);
}
