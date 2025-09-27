
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShadesOfPurple;

impl SixColorsTwoRowsStyler for ShadesOfPurple {
    const BACKGROUND: Color = Color::from_u32(0x001e1d40);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3D60);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f92a1c),
        Color::from_u32(0x0043d426),
        Color::from_u32(0x00f1d000),
        Color::from_u32(0x006871ff),
        Color::from_u32(0x00ff77ff),
        Color::from_u32(0x0079e8fb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d90429),
        Color::from_u32(0x003ad900),
        Color::from_u32(0x00ffe700),
        Color::from_u32(0x006943ff),
        Color::from_u32(0x00ff2c70),
        Color::from_u32(0x0000c5c7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242346), Color::from_u32(0x002A294C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DAB000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFB700);

    fn id(&self) -> &str {
        "shades_of_purple"
    }

    fn title(&self) -> &str {
        "ShadesOfPurple"
    }
}
