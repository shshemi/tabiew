
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Zenburned;

impl SixColorsTwoRowsStyler for Zenburned {
    const BACKGROUND: Color = Color::from_u32(0x00404040);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00606060);
    const FOREGROUND: Color = Color::from_u32(0x00f0e4cf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00404040);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec8685),
        Color::from_u32(0x008bae68),
        Color::from_u32(0x00d68c67),
        Color::from_u32(0x0061abda),
        Color::from_u32(0x00cf86c1),
        Color::from_u32(0x0065b8c1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e3716e),
        Color::from_u32(0x00819b69),
        Color::from_u32(0x00b77e64),
        Color::from_u32(0x006099c0),
        Color::from_u32(0x00b279a7),
        Color::from_u32(0x0066a5ad),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00464646), Color::from_u32(0x004C4C4C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D3CABB);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B3413E);
}
