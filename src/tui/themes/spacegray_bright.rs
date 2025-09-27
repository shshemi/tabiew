
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SpacegrayBright;

impl SixColorsTwoRowsStyler for SpacegrayBright {
    const BACKGROUND: Color = Color::from_u32(0x002a2e3a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004A4E5A);
    const FOREGROUND: Color = Color::from_u32(0x00f3f3f3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00080808);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bc5553),
        Color::from_u32(0x00a0b56c),
        Color::from_u32(0x00f6c987),
        Color::from_u32(0x007baec1),
        Color::from_u32(0x00b98aae),
        Color::from_u32(0x0085c9b8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bc5553),
        Color::from_u32(0x00a0b56c),
        Color::from_u32(0x00f6c987),
        Color::from_u32(0x007baec1),
        Color::from_u32(0x00b98aae),
        Color::from_u32(0x0085c9b8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00303440), Color::from_u32(0x00363A46)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A6A6A6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C69957);

    fn id(&self) -> &str {
        "spacegray_bright"
    }

    fn title(&self) -> &str {
        "SpacegrayBright"
    }
}
