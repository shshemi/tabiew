
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Deep;

impl SixColorsTwoRowsStyler for Deep {
    const BACKGROUND: Color = Color::from_u32(0x00090909);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00292929);
    const FOREGROUND: Color = Color::from_u32(0x00cdcdcd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb0007),
        Color::from_u32(0x0022ff18),
        Color::from_u32(0x00fedc2b),
        Color::from_u32(0x009fa9ff),
        Color::from_u32(0x00e09aff),
        Color::from_u32(0x008df9ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d70005),
        Color::from_u32(0x001cd915),
        Color::from_u32(0x00d9bd26),
        Color::from_u32(0x005665ff),
        Color::from_u32(0x00b052da),
        Color::from_u32(0x0050d2da),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000F0F0F), Color::from_u32(0x00151515)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B0B0B0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A98D00);

    fn id(&self) -> &str {
        "deep"
    }

    fn title(&self) -> &str {
        "Deep"
    }
}
