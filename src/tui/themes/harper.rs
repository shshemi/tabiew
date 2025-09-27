
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Harper;

impl SixColorsTwoRowsStyler for Harper {
    const BACKGROUND: Color = Color::from_u32(0x00010101);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00212121);
    const FOREGROUND: Color = Color::from_u32(0x00a8a49d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00010101);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f8b63f),
        Color::from_u32(0x007fb5e1),
        Color::from_u32(0x00d6da25),
        Color::from_u32(0x00489e48),
        Color::from_u32(0x00b296c6),
        Color::from_u32(0x00f5bfd7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f8b63f),
        Color::from_u32(0x007fb5e1),
        Color::from_u32(0x00d6da25),
        Color::from_u32(0x00489e48),
        Color::from_u32(0x00b296c6),
        Color::from_u32(0x00f5bfd7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00070707), Color::from_u32(0x000D0D0D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0088847D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C8860F);

    fn id(&self) -> &str {
        "harper"
    }

    fn title(&self) -> &str {
        "Harper"
    }
}
