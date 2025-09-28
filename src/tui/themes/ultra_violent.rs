
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct UltraViolent;

impl SixColorsTwoRowsStyler for UltraViolent {
    const BACKGROUND: Color = Color::from_u32(0x00242728);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00444748);
    const FOREGROUND: Color = Color::from_u32(0x00c1c1c1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00242728);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb58b4),
        Color::from_u32(0x00deff8c),
        Color::from_u32(0x00ebe087),
        Color::from_u32(0x007fecff),
        Color::from_u32(0x00e681ff),
        Color::from_u32(0x0069fcd3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0090),
        Color::from_u32(0x00b6ff00),
        Color::from_u32(0x00fff727),
        Color::from_u32(0x0047e0fb),
        Color::from_u32(0x00d731ff),
        Color::from_u32(0x000effbb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A2D2E), Color::from_u32(0x00303334)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A1A1A1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFC700);
}
