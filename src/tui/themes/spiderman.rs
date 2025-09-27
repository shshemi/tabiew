
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Spiderman;

impl SixColorsTwoRowsStyler for Spiderman {
    const BACKGROUND: Color = Color::from_u32(0x001b1d1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3D3E);
    const FOREGROUND: Color = Color::from_u32(0x00e3e3e3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0325),
        Color::from_u32(0x00ff3338),
        Color::from_u32(0x00fe3a35),
        Color::from_u32(0x001d50ff),
        Color::from_u32(0x00747cff),
        Color::from_u32(0x006184ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e60813),
        Color::from_u32(0x00e22928),
        Color::from_u32(0x00e24756),
        Color::from_u32(0x002c3fff),
        Color::from_u32(0x002435db),
        Color::from_u32(0x003256ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212324), Color::from_u32(0x0027292A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x000C1FDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B60000);

    fn id(&self) -> &str {
        "spiderman"
    }

    fn title(&self) -> &str {
        "Spiderman"
    }
}
