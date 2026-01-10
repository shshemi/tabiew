use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Aura;

impl SixColorsTwoRowsStyler for Aura {
    const BACKGROUND: Color = Color::from_u32(0x0015141b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0035343B);
    const FOREGROUND: Color = Color::from_u32(0x00edecee);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00110f18);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffca85),
        Color::from_u32(0x00a277ff),
        Color::from_u32(0x00ffca85),
        Color::from_u32(0x00a277ff),
        Color::from_u32(0x00a277ff),
        Color::from_u32(0x0061ffca),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6767),
        Color::from_u32(0x0061ffca),
        Color::from_u32(0x00ffca85),
        Color::from_u32(0x00a277ff),
        Color::from_u32(0x00a277ff),
        Color::from_u32(0x0061ffca),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001B1A21), Color::from_u32(0x00212027)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008257DF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3737);
}
