
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Box;

impl SixColorsTwoRowsStyler for Box {
    const BACKGROUND: Color = Color::from_u32(0x00141d2b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343D4B);
    const FOREGROUND: Color = Color::from_u32(0x009fef00);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f2201f),
        Color::from_u32(0x0023fd00),
        Color::from_u32(0x00fffd00),
        Color::from_u32(0x001a8fff),
        Color::from_u32(0x00fd28ff),
        Color::from_u32(0x0014ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc0403),
        Color::from_u32(0x0019cb00),
        Color::from_u32(0x00cecb00),
        Color::from_u32(0x000d73cc),
        Color::from_u32(0x00cb1ed1),
        Color::from_u32(0x000dcdcd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A2331), Color::from_u32(0x00202937)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007FCF00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009E9B00);

    fn id(&self) -> &str {
        "box"
    }

    fn title(&self) -> &str {
        "Box"
    }
}
