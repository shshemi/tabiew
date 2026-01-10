use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Detuned;

impl SixColorsTwoRowsStyler for Detuned {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00c7c7c7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00171717);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fa80ac),
        Color::from_u32(0x00bde371),
        Color::from_u32(0x00fff27f),
        Color::from_u32(0x0000beff),
        Color::from_u32(0x00be9eff),
        Color::from_u32(0x005ed7ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fe4386),
        Color::from_u32(0x00a6e32d),
        Color::from_u32(0x00e6da73),
        Color::from_u32(0x000094d9),
        Color::from_u32(0x009b37ff),
        Color::from_u32(0x0050b7d9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE1356);
}
