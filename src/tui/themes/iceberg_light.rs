
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IcebergLight;

impl SixColorsTwoRowsStyler for IcebergLight {
    const BACKGROUND: Color = Color::from_u32(0x00e8e9ec);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0033374c);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00dcdfe7);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc3768),
        Color::from_u32(0x00598030),
        Color::from_u32(0x00b6662d),
        Color::from_u32(0x0022478e),
        Color::from_u32(0x006845ad),
        Color::from_u32(0x00327698),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc517a),
        Color::from_u32(0x00668e3d),
        Color::from_u32(0x00c57339),
        Color::from_u32(0x002d539e),
        Color::from_u32(0x007759b4),
        Color::from_u32(0x003f83a6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00EEEFF2), Color::from_u32(0x00F4F5F8)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0013172C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009C214A);

    fn id(&self) -> &str {
        "iceberg_light"
    }

    fn title(&self) -> &str {
        "IcebergLight"
    }
}
