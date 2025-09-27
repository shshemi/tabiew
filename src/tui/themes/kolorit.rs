
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Kolorit;

impl SixColorsTwoRowsStyler for Kolorit {
    const BACKGROUND: Color = Color::from_u32(0x001d1a1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3A3E);
    const FOREGROUND: Color = Color::from_u32(0x00efecec);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d1a1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5b82),
        Color::from_u32(0x0047d7a1),
        Color::from_u32(0x00e8e562),
        Color::from_u32(0x005db4ee),
        Color::from_u32(0x00da6cda),
        Color::from_u32(0x0057e9eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5b82),
        Color::from_u32(0x0047d7a1),
        Color::from_u32(0x00e8e562),
        Color::from_u32(0x005db4ee),
        Color::from_u32(0x00da6cda),
        Color::from_u32(0x0057e9eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232024), Color::from_u32(0x0029262A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF2B52);

    fn id(&self) -> &str {
        "kolorit"
    }

    fn title(&self) -> &str {
        "Kolorit"
    }
}
