
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CatppuccinLatte;

impl SixColorsTwoRowsStyler for CatppuccinLatte {
    const BACKGROUND: Color = Color::from_u32(0x00eff1f5);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x004c4f69);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005c5f77);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de293e),
        Color::from_u32(0x0049af3d),
        Color::from_u32(0x00eea02d),
        Color::from_u32(0x00456eff),
        Color::from_u32(0x00fe85d8),
        Color::from_u32(0x002d9fa8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d20f39),
        Color::from_u32(0x0040a02b),
        Color::from_u32(0x00df8e1d),
        Color::from_u32(0x001e66f5),
        Color::from_u32(0x00ea76cb),
        Color::from_u32(0x00179299),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F5F7FB), Color::from_u32(0x00FBFDFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BC6A58);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA469B);

    fn id(&self) -> &str {
        "catppuccin_latte"
    }

    fn title(&self) -> &str {
        "CatppuccinLatte"
    }
}
