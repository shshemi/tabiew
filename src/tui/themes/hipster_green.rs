
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HipsterGreen;

impl SixColorsTwoRowsStyler for HipsterGreen {
    const BACKGROUND: Color = Color::from_u32(0x00100b05);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00302B25);
    const FOREGROUND: Color = Color::from_u32(0x0084c138);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e50000),
        Color::from_u32(0x0086a93e),
        Color::from_u32(0x00e5e500),
        Color::from_u32(0x000000ff),
        Color::from_u32(0x00e500e5),
        Color::from_u32(0x0000e5e5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b6214a),
        Color::from_u32(0x0000a600),
        Color::from_u32(0x00bfbf00),
        Color::from_u32(0x00246eb2),
        Color::from_u32(0x00b200b2),
        Color::from_u32(0x0000a6b2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0016110B), Color::from_u32(0x001C1711)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0003DF00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x008F8F00);

    fn id(&self) -> &str {
        "hipster_green"
    }

    fn title(&self) -> &str {
        "HipsterGreen"
    }
}
