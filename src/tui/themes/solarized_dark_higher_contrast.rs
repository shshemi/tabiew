
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SolarizedDarkHigherContrast;

impl SixColorsTwoRowsStyler for SolarizedDarkHigherContrast {
    const BACKGROUND: Color = Color::from_u32(0x00001e27);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00203E47);
    const FOREGROUND: Color = Color::from_u32(0x009cc2c3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00002831);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f5163b),
        Color::from_u32(0x0051ef84),
        Color::from_u32(0x00b27e28),
        Color::from_u32(0x00178ec8),
        Color::from_u32(0x00e24d8e),
        Color::from_u32(0x0000b39e),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d11c24),
        Color::from_u32(0x006cbe6c),
        Color::from_u32(0x00a57706),
        Color::from_u32(0x002176c7),
        Color::from_u32(0x00c61c6f),
        Color::from_u32(0x00259286),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0006242D), Color::from_u32(0x000C2A33)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D32B00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A10000);
}
