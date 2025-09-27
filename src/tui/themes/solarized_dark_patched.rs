
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SolarizedDarkPatched;

impl SixColorsTwoRowsStyler for SolarizedDarkPatched {
    const BACKGROUND: Color = Color::from_u32(0x00001e27);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00203E47);
    const FOREGROUND: Color = Color::from_u32(0x00708284);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00002831);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bd3613),
        Color::from_u32(0x00475b62),
        Color::from_u32(0x00536870),
        Color::from_u32(0x00708284),
        Color::from_u32(0x005956ba),
        Color::from_u32(0x00819090),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d11c24),
        Color::from_u32(0x00738a05),
        Color::from_u32(0x00a57706),
        Color::from_u32(0x002176c7),
        Color::from_u32(0x00c61c6f),
        Color::from_u32(0x00259286),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0006242D), Color::from_u32(0x000C2A33)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A10000);

    fn id(&self) -> &str {
        "solarized_dark_patched"
    }

    fn title(&self) -> &str {
        "SolarizedDarkPatched"
    }
}
