
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VioletLight;

impl SixColorsTwoRowsStyler for VioletLight {
    const BACKGROUND: Color = Color::from_u32(0x00fcf4dc);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFC);
    const FOREGROUND: Color = Color::from_u32(0x00536870);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0056595c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bd3613),
        Color::from_u32(0x00738a04),
        Color::from_u32(0x00a57705),
        Color::from_u32(0x002176c7),
        Color::from_u32(0x00c61c6f),
        Color::from_u32(0x00259286),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94c22),
        Color::from_u32(0x0085981c),
        Color::from_u32(0x00b4881d),
        Color::from_u32(0x002e8bce),
        Color::from_u32(0x00d13a82),
        Color::from_u32(0x0032a198),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFAE2), Color::from_u32(0x00FFFFE8)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00334850);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A10A52);

    fn id(&self) -> &str {
        "violet_light"
    }

    fn title(&self) -> &str {
        "VioletLight"
    }
}
