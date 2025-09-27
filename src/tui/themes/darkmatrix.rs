
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Darkmatrix;

impl SixColorsTwoRowsStyler for Darkmatrix {
    const BACKGROUND: Color = Color::from_u32(0x00070c0e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00272C2E);
    const FOREGROUND: Color = Color::from_u32(0x003e5715);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00091013);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0000733d),
        Color::from_u32(0x0090d762),
        Color::from_u32(0x00e2e500),
        Color::from_u32(0x0046d8b8),
        Color::from_u32(0x004a3059),
        Color::from_u32(0x0012545a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00006536),
        Color::from_u32(0x006fa64c),
        Color::from_u32(0x007e8000),
        Color::from_u32(0x002c9a84),
        Color::from_u32(0x00452d53),
        Color::from_u32(0x00114d53),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000D1214), Color::from_u32(0x0013181A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007F884E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x004E5000);

    fn id(&self) -> &str {
        "darkmatrix"
    }

    fn title(&self) -> &str {
        "Darkmatrix"
    }
}
