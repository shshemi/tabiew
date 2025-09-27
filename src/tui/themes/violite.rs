
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Violite;

impl SixColorsTwoRowsStyler for Violite {
    const BACKGROUND: Color = Color::from_u32(0x00241c36);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00443C56);
    const FOREGROUND: Color = Color::from_u32(0x00eef4f6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00241c36);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef8f8f),
        Color::from_u32(0x009fefbf),
        Color::from_u32(0x00efe78f),
        Color::from_u32(0x00b78fef),
        Color::from_u32(0x00ef8fcf),
        Color::from_u32(0x009fefef),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec7979),
        Color::from_u32(0x0079ecb3),
        Color::from_u32(0x00ece279),
        Color::from_u32(0x00a979ec),
        Color::from_u32(0x00ec79ec),
        Color::from_u32(0x0079ecec),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A223C), Color::from_u32(0x00302842)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CED4D6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BC4949);

    fn id(&self) -> &str {
        "violite"
    }

    fn title(&self) -> &str {
        "Violite"
    }
}
