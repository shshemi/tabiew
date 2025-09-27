
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TerminalBasicDark;

impl SixColorsTwoRowsStyler for TerminalBasicDark {
    const BACKGROUND: Color = Color::from_u32(0x001d1e1d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3E3D);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb5a3a),
        Color::from_u32(0x0077ea51),
        Color::from_u32(0x00efef53),
        Color::from_u32(0x00d09af9),
        Color::from_u32(0x00eb5af7),
        Color::from_u32(0x0078f1f2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c65339),
        Color::from_u32(0x006ac44b),
        Color::from_u32(0x00b8b74a),
        Color::from_u32(0x006444ed),
        Color::from_u32(0x00d357db),
        Color::from_u32(0x0069c1cf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232423), Color::from_u32(0x00292A29)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007D7D7D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A327AB);

    fn id(&self) -> &str {
        "terminal_basic_dark"
    }

    fn title(&self) -> &str {
        "TerminalBasicDark"
    }
}
