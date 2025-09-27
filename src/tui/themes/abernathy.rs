
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Abernathy;

impl SixColorsTwoRowsStyler for Abernathy {
    const BACKGROUND: Color = Color::from_u32(0x00111416);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313436);
    const FOREGROUND: Color = Color::from_u32(0x00eeeeec);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x0000ff00),
        Color::from_u32(0x00ffff00),
        Color::from_u32(0x0011b5f6),
        Color::from_u32(0x00ff00ff),
        Color::from_u32(0x0000ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cd0000),
        Color::from_u32(0x0000cd00),
        Color::from_u32(0x00cdcd00),
        Color::from_u32(0x001093f5),
        Color::from_u32(0x00cd00cd),
        Color::from_u32(0x0000cdcd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00171A1C), Color::from_u32(0x001D2022)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009D0000);

    fn id(&self) -> &str {
        "abernathy"
    }

    fn title(&self) -> &str {
        "Abernathy"
    }
}
