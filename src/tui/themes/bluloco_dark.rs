
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlulocoDark;

impl SixColorsTwoRowsStyler for BlulocoDark {
    const BACKGROUND: Color = Color::from_u32(0x00282c34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C54);
    const FOREGROUND: Color = Color::from_u32(0x00b9c0cb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0041444d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6480),
        Color::from_u32(0x003fc56b),
        Color::from_u32(0x00f9c859),
        Color::from_u32(0x0010b1fe),
        Color::from_u32(0x00ff78f8),
        Color::from_u32(0x005fb9bc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc2f52),
        Color::from_u32(0x0025a45c),
        Color::from_u32(0x00ff936a),
        Color::from_u32(0x003476ff),
        Color::from_u32(0x007a82da),
        Color::from_u32(0x004483aa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323A), Color::from_u32(0x00343840)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFAC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF633A);

    fn id(&self) -> &str {
        "bluloco_dark"
    }

    fn title(&self) -> &str {
        "BlulocoDark"
    }
}
