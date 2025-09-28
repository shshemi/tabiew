
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MaterialDark;

impl SixColorsTwoRowsStyler for MaterialDark {
    const BACKGROUND: Color = Color::from_u32(0x00232322);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00434342);
    const FOREGROUND: Color = Color::from_u32(0x00e5e5e5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00212121);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e83b3f),
        Color::from_u32(0x007aba3a),
        Color::from_u32(0x00ffea2e),
        Color::from_u32(0x0054a4f3),
        Color::from_u32(0x00aa4dbc),
        Color::from_u32(0x0026bbd1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b7141f),
        Color::from_u32(0x00457b24),
        Color::from_u32(0x00f6981e),
        Color::from_u32(0x00134eb2),
        Color::from_u32(0x00560088),
        Color::from_u32(0x000e717c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292928), Color::from_u32(0x002F2F2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00008FAA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C66800);
}
