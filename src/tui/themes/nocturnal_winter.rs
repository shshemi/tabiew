
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NocturnalWinter;

impl SixColorsTwoRowsStyler for NocturnalWinter {
    const BACKGROUND: Color = Color::from_u32(0x000d0d17);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D37);
    const FOREGROUND: Color = Color::from_u32(0x00e6e5e5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004d4d4d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f16d86),
        Color::from_u32(0x000ae78d),
        Color::from_u32(0x00fffc67),
        Color::from_u32(0x006096ff),
        Color::from_u32(0x00ff78a2),
        Color::from_u32(0x000ae78d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f12d52),
        Color::from_u32(0x0009cd7e),
        Color::from_u32(0x00f5f17a),
        Color::from_u32(0x003182e0),
        Color::from_u32(0x00ff2b6d),
        Color::from_u32(0x0009c87a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0013131D), Color::from_u32(0x00191923)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C6C5C5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF003D);

    fn id(&self) -> &str {
        "nocturnal_winter"
    }

    fn title(&self) -> &str {
        "NocturnalWinter"
    }
}
