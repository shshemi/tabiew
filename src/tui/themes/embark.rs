
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Embark;

impl SixColorsTwoRowsStyler for Embark {
    const BACKGROUND: Color = Color::from_u32(0x001e1c31);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3C51);
    const FOREGROUND: Color = Color::from_u32(0x00eeffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001e1c31);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f02e6e),
        Color::from_u32(0x002ce592),
        Color::from_u32(0x00ffb378),
        Color::from_u32(0x001da0e2),
        Color::from_u32(0x00a742ea),
        Color::from_u32(0x0063f2f1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f0719b),
        Color::from_u32(0x00a1efd3),
        Color::from_u32(0x00ffe9aa),
        Color::from_u32(0x0057c7ff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x0087dfeb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242237), Color::from_u32(0x002A283D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0081CFB3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFB97A);
}
