
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IcebergDark;

impl SixColorsTwoRowsStyler for IcebergDark {
    const BACKGROUND: Color = Color::from_u32(0x00161821);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00363841);
    const FOREGROUND: Color = Color::from_u32(0x00c6c8d1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001e2132);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e98989),
        Color::from_u32(0x00c0ca8e),
        Color::from_u32(0x00e9b189),
        Color::from_u32(0x0091acd1),
        Color::from_u32(0x00ada0d3),
        Color::from_u32(0x0095c4ce),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e27878),
        Color::from_u32(0x00b4be82),
        Color::from_u32(0x00e2a478),
        Color::from_u32(0x0084a0c6),
        Color::from_u32(0x00a093c7),
        Color::from_u32(0x0089b8c2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1E27), Color::from_u32(0x0022242D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A6A8B1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B24848);

    fn id(&self) -> &str {
        "iceberg_dark"
    }

    fn title(&self) -> &str {
        "IcebergDark"
    }
}
