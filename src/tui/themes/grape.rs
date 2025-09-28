
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Grape;

impl SixColorsTwoRowsStyler for Grape {
    const BACKGROUND: Color = Color::from_u32(0x00171423);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00373443);
    const FOREGROUND: Color = Color::from_u32(0x009f9fa1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002d283f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f0729a),
        Color::from_u32(0x0053aa5e),
        Color::from_u32(0x00b2dc87),
        Color::from_u32(0x00a9bcec),
        Color::from_u32(0x00ad81c2),
        Color::from_u32(0x009de3eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ed2261),
        Color::from_u32(0x001fa91b),
        Color::from_u32(0x008ddc20),
        Color::from_u32(0x00487df4),
        Color::from_u32(0x008d35c9),
        Color::from_u32(0x003bdeed),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001D1A29), Color::from_u32(0x0023202F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008268D7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BD0031);
}
