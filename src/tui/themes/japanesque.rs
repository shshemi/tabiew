
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Japanesque;

impl SixColorsTwoRowsStyler for Japanesque {
    const BACKGROUND: Color = Color::from_u32(0x001e1e1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3E3E);
    const FOREGROUND: Color = Color::from_u32(0x00f7f6ec);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00343935);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d18fa6),
        Color::from_u32(0x00767f2c),
        Color::from_u32(0x0078592f),
        Color::from_u32(0x00135979),
        Color::from_u32(0x00604291),
        Color::from_u32(0x0076bbca),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cf3f61),
        Color::from_u32(0x007bb75b),
        Color::from_u32(0x00e9b32a),
        Color::from_u32(0x004c9ad4),
        Color::from_u32(0x00a57fc4),
        Color::from_u32(0x00389aad),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242424), Color::from_u32(0x002A2A2A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CDAF2F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B98300);

    fn id(&self) -> &str {
        "japanesque"
    }

    fn title(&self) -> &str {
        "Japanesque"
    }
}
