
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ciapre;

impl SixColorsTwoRowsStyler for Ciapre {
    const BACKGROUND: Color = Color::from_u32(0x00191c27);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393C47);
    const FOREGROUND: Color = Color::from_u32(0x00aea47a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00181818);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ac3835),
        Color::from_u32(0x00a6a75d),
        Color::from_u32(0x00dcdf7c),
        Color::from_u32(0x003097c6),
        Color::from_u32(0x00d33061),
        Color::from_u32(0x00f3dbb2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00810009),
        Color::from_u32(0x0048513b),
        Color::from_u32(0x00cc8b3f),
        Color::from_u32(0x00576d8c),
        Color::from_u32(0x00724d7c),
        Color::from_u32(0x005c4f4b),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F222D), Color::from_u32(0x00252833)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0072603B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009C5B0F);
}
