
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Tomorrow;

impl SixColorsTwoRowsStyler for Tomorrow {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x004d4d4c);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c82829),
        Color::from_u32(0x00718c00),
        Color::from_u32(0x00eab700),
        Color::from_u32(0x004271ae),
        Color::from_u32(0x008959a8),
        Color::from_u32(0x003e999f),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c82829),
        Color::from_u32(0x00718c00),
        Color::from_u32(0x00eab700),
        Color::from_u32(0x004271ae),
        Color::from_u32(0x008959a8),
        Color::from_u32(0x003e999f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA8700);

    fn id(&self) -> &str {
        "tomorrow"
    }

    fn title(&self) -> &str {
        "Tomorrow"
    }
}
