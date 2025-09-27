
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Broadcast;

impl SixColorsTwoRowsStyler for Broadcast {
    const BACKGROUND: Color = Color::from_u32(0x002b2b2b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004B4B4B);
    const FOREGROUND: Color = Color::from_u32(0x00e6e1dc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff7b6b),
        Color::from_u32(0x0083d182),
        Color::from_u32(0x00ffff7c),
        Color::from_u32(0x009fcef0),
        Color::from_u32(0x00ffffff),
        Color::from_u32(0x00a0cef0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00da4939),
        Color::from_u32(0x00519f50),
        Color::from_u32(0x00ffd24a),
        Color::from_u32(0x006d9cbe),
        Color::from_u32(0x00d0d0ff),
        Color::from_u32(0x006e9cbe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00313131), Color::from_u32(0x00373737)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA21A);

    fn id(&self) -> &str {
        "broadcast"
    }

    fn title(&self) -> &str {
        "Broadcast"
    }
}
