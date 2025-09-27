
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SolarizedDarcula;

impl SixColorsTwoRowsStyler for SolarizedDarcula {
    const BACKGROUND: Color = Color::from_u32(0x003d3f41);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x005D5F61);
    const FOREGROUND: Color = Color::from_u32(0x00d2d8d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0025292a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f24840),
        Color::from_u32(0x00629655),
        Color::from_u32(0x00b68800),
        Color::from_u32(0x002075c7),
        Color::from_u32(0x00797fd4),
        Color::from_u32(0x0015968d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f24840),
        Color::from_u32(0x00629655),
        Color::from_u32(0x00b68800),
        Color::from_u32(0x002075c7),
        Color::from_u32(0x00797fd4),
        Color::from_u32(0x0015968d),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00434547), Color::from_u32(0x00494B4D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C21810);

    fn id(&self) -> &str {
        "solarized_darcula"
    }

    fn title(&self) -> &str {
        "SolarizedDarcula"
    }
}
