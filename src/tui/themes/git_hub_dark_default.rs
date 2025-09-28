
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubDarkDefault;

impl SixColorsTwoRowsStyler for GithubDarkDefault {
    const BACKGROUND: Color = Color::from_u32(0x000d1117);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002D3137);
    const FOREGROUND: Color = Color::from_u32(0x00e6edf3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00484f58);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffa198),
        Color::from_u32(0x0056d364),
        Color::from_u32(0x00e3b341),
        Color::from_u32(0x0079c0ff),
        Color::from_u32(0x00d2a8ff),
        Color::from_u32(0x0056d4dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff7b72),
        Color::from_u32(0x003fb950),
        Color::from_u32(0x00d29922),
        Color::from_u32(0x0058a6ff),
        Color::from_u32(0x00bc8cff),
        Color::from_u32(0x0039c5cf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0013171D), Color::from_u32(0x00191D23)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x000F61D7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF4B42);
}
