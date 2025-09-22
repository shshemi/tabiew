use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Argonaut;

impl SixColorsTwoRowsStyler for Argonaut {
    const BACKGROUND: Color = Color::from_u32(0x0001030b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0023252d);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0043454d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0011131b), Color::from_u32(0x0001030b)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00204a5b);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00dd0000);

    fn id(&self) -> &str {
        "argonaut"
    }

    fn title(&self) -> &str {
        "Argonaut"
    }
}
