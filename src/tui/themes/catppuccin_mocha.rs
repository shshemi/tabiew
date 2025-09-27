
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CatppuccinMocha;

impl SixColorsTwoRowsStyler for CatppuccinMocha {
    const BACKGROUND: Color = Color::from_u32(0x001e1e2e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3E4E);
    const FOREGROUND: Color = Color::from_u32(0x00cdd6f4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0045475a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f37799),
        Color::from_u32(0x0089d88b),
        Color::from_u32(0x00ebd391),
        Color::from_u32(0x0074a8fc),
        Color::from_u32(0x00f2aede),
        Color::from_u32(0x006bd7ca),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x00f9e2af),
        Color::from_u32(0x0089b4fa),
        Color::from_u32(0x00f5c2e7),
        Color::from_u32(0x0094e2d5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242434), Color::from_u32(0x002A2A3A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D5C0BC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C9B27F);

    fn id(&self) -> &str {
        "catppuccin_mocha"
    }

    fn title(&self) -> &str {
        "CatppuccinMocha"
    }
}
