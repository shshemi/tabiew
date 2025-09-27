
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OceanicMaterial;

impl SixColorsTwoRowsStyler for OceanicMaterial {
    const BACKGROUND: Color = Color::from_u32(0x001c262b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C464B);
    const FOREGROUND: Color = Color::from_u32(0x00c2c8d7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dc5c60),
        Color::from_u32(0x0070be71),
        Color::from_u32(0x00fff163),
        Color::from_u32(0x0054a4f3),
        Color::from_u32(0x00aa4dbc),
        Color::from_u32(0x0042c7da),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ee2b2a),
        Color::from_u32(0x0040a33f),
        Color::from_u32(0x00ffea2e),
        Color::from_u32(0x001e80f0),
        Color::from_u32(0x008800a0),
        Color::from_u32(0x0016afca),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00222C31), Color::from_u32(0x00283237)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009398A3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFBA00);

    fn id(&self) -> &str {
        "oceanic_material"
    }

    fn title(&self) -> &str {
        "OceanicMaterial"
    }
}
