
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OperatorMonoDark;

impl SixColorsTwoRowsStyler for OperatorMonoDark {
    const BACKGROUND: Color = Color::from_u32(0x00191919);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393939);
    const FOREGROUND: Color = Color::from_u32(0x00c3cac2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005a5a5a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c37d62),
        Color::from_u32(0x0083d0a2),
        Color::from_u32(0x00fdfdc5),
        Color::from_u32(0x0089d3f6),
        Color::from_u32(0x00ff2c7a),
        Color::from_u32(0x0082eada),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ca372d),
        Color::from_u32(0x004d7b3a),
        Color::from_u32(0x00d4d697),
        Color::from_u32(0x004387cf),
        Color::from_u32(0x00b86cb4),
        Color::from_u32(0x0072d5c6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1F1F), Color::from_u32(0x00252525)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DCBC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A4A667);
}
