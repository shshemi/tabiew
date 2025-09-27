
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PiattoLight;

impl SixColorsTwoRowsStyler for PiattoLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00414141);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00414141);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00db3365),
        Color::from_u32(0x00829429),
        Color::from_u32(0x00cd6f34),
        Color::from_u32(0x003c5ea8),
        Color::from_u32(0x00a454b2),
        Color::from_u32(0x00829429),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b23771),
        Color::from_u32(0x0066781e),
        Color::from_u32(0x00cd6f34),
        Color::from_u32(0x003c5ea8),
        Color::from_u32(0x00a454b2),
        Color::from_u32(0x0066781e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x003E57A8);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009D3F04);

    fn id(&self) -> &str {
        "piatto_light"
    }

    fn title(&self) -> &str {
        "PiattoLight"
    }
}
