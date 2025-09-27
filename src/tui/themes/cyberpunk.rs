
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Cyberpunk;

impl SixColorsTwoRowsStyler for Cyberpunk {
    const BACKGROUND: Color = Color::from_u32(0x00332a57);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00534A77);
    const FOREGROUND: Color = Color::from_u32(0x00e5e5e5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8aa4),
        Color::from_u32(0x0021f6bc),
        Color::from_u32(0x00fff787),
        Color::from_u32(0x001bccfd),
        Color::from_u32(0x00e6aefe),
        Color::from_u32(0x0099d6fc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff7092),
        Color::from_u32(0x0000fbac),
        Color::from_u32(0x00fffa6a),
        Color::from_u32(0x0000bfff),
        Color::from_u32(0x00df95ff),
        Color::from_u32(0x0086cbfe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0039305D), Color::from_u32(0x003F3663)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0001D69C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFCA3A);

    fn id(&self) -> &str {
        "cyberpunk"
    }

    fn title(&self) -> &str {
        "Cyberpunk"
    }
}
