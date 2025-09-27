
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Chalkboard;

impl SixColorsTwoRowsStyler for Chalkboard {
    const BACKGROUND: Color = Color::from_u32(0x0029262f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0049464F);
    const FOREGROUND: Color = Color::from_u32(0x00d9e6f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dbaaaa),
        Color::from_u32(0x00aadbaa),
        Color::from_u32(0x00dadbaa),
        Color::from_u32(0x00aaaadb),
        Color::from_u32(0x00dbaada),
        Color::from_u32(0x00aadadb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c37372),
        Color::from_u32(0x0072c373),
        Color::from_u32(0x00c2c372),
        Color::from_u32(0x007372c3),
        Color::from_u32(0x00c372c2),
        Color::from_u32(0x0072c2c3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F2C35), Color::from_u32(0x0035323B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B9C6D2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00934342);

    fn id(&self) -> &str {
        "chalkboard"
    }

    fn title(&self) -> &str {
        "Chalkboard"
    }
}
