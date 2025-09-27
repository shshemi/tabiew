
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RetroLegends;

impl SixColorsTwoRowsStyler for RetroLegends {
    const BACKGROUND: Color = Color::from_u32(0x000d0d0d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2D);
    const FOREGROUND: Color = Color::from_u32(0x0045eb45);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00262626);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6666),
        Color::from_u32(0x0059ff59),
        Color::from_u32(0x00ffd933),
        Color::from_u32(0x004c80ff),
        Color::from_u32(0x00e666ff),
        Color::from_u32(0x0059e6ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00de5454),
        Color::from_u32(0x0045eb45),
        Color::from_u32(0x00f7bf2b),
        Color::from_u32(0x004066f2),
        Color::from_u32(0x00bf4cf2),
        Color::from_u32(0x0040d9e6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00131313), Color::from_u32(0x00191919)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0025CB25);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C78F00);

    fn id(&self) -> &str {
        "retro_legends"
    }

    fn title(&self) -> &str {
        "RetroLegends"
    }
}
