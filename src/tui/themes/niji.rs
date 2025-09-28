
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Niji;

impl SixColorsTwoRowsStyler for Niji {
    const BACKGROUND: Color = Color::from_u32(0x00141515);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343535);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffb7b7),
        Color::from_u32(0x00c1ffae),
        Color::from_u32(0x00fcffb8),
        Color::from_u32(0x008efff3),
        Color::from_u32(0x00ffa2ed),
        Color::from_u32(0x00bcffc7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d23e08),
        Color::from_u32(0x0054ca74),
        Color::from_u32(0x00fff700),
        Color::from_u32(0x002ab9ff),
        Color::from_u32(0x00ff50da),
        Color::from_u32(0x001ef9f5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1B1B), Color::from_u32(0x00202121)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFA643);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFC700);
}
