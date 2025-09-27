
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KittyLowContrast;

impl SixColorsTwoRowsStyler for KittyLowContrast {
    const BACKGROUND: Color = Color::from_u32(0x00333333);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00535353);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00393939), Color::from_u32(0x003F3F3F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ACACAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009E9B00);

    fn id(&self) -> &str {
        "kitty_low_contrast"
    }

    fn title(&self) -> &str {
        "KittyLowContrast"
    }
}
