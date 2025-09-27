
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BuiltinSolarizedLight;

impl SixColorsTwoRowsStyler for BuiltinSolarizedLight {
    const BACKGROUND: Color = Color::from_u32(0x00fdf6e3);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00657b83);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00073642);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cb4b16),
        Color::from_u32(0x00586e75),
        Color::from_u32(0x00657b83),
        Color::from_u32(0x00839496),
        Color::from_u32(0x006c71c4),
        Color::from_u32(0x0093a1a1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00dc322f),
        Color::from_u32(0x00859900),
        Color::from_u32(0x00b58900),
        Color::from_u32(0x00268bd2),
        Color::from_u32(0x00d33682),
        Color::from_u32(0x002aa198),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFCE9), Color::from_u32(0x00FFFFEF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00455B63);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AC0200);

    fn id(&self) -> &str {
        "builtin_solarized_light"
    }

    fn title(&self) -> &str {
        "BuiltinSolarizedLight"
    }
}
