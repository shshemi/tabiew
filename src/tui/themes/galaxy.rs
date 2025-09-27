
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Galaxy;

impl SixColorsTwoRowsStyler for Galaxy {
    const BACKGROUND: Color = Color::from_u32(0x001d2837);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D4857);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fa8c8f),
        Color::from_u32(0x0035bb9a),
        Color::from_u32(0x00ffff55),
        Color::from_u32(0x00589df6),
        Color::from_u32(0x00e75699),
        Color::from_u32(0x003979bc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f9555f),
        Color::from_u32(0x0021b089),
        Color::from_u32(0x00fef02a),
        Color::from_u32(0x00589df6),
        Color::from_u32(0x00944d95),
        Color::from_u32(0x001f9ee7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232E3D), Color::from_u32(0x00293443)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CEC000);

    fn id(&self) -> &str {
        "galaxy"
    }

    fn title(&self) -> &str {
        "Galaxy"
    }
}
