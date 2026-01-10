use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Synthwave;

impl SixColorsTwoRowsStyler for Synthwave {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00dad9c7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f841a0),
        Color::from_u32(0x0025c141),
        Color::from_u32(0x00fdf454),
        Color::from_u32(0x002f9ded),
        Color::from_u32(0x00f97137),
        Color::from_u32(0x0019cde6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f6188f),
        Color::from_u32(0x001ebb2b),
        Color::from_u32(0x00fdf834),
        Color::from_u32(0x002186ec),
        Color::from_u32(0x00f85a21),
        Color::from_u32(0x0012c3e2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0000ADC6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CDC804);
}
