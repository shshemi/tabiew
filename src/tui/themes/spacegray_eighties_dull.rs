use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SpacegrayEightiesDull;

impl SixColorsTwoRowsStyler for SpacegrayEightiesDull {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00c9c6bc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0015171c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec5f67),
        Color::from_u32(0x0089e986),
        Color::from_u32(0x00fec254),
        Color::from_u32(0x005486c0),
        Color::from_u32(0x00bf83c1),
        Color::from_u32(0x0058c2c1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b24a56),
        Color::from_u32(0x0092b477),
        Color::from_u32(0x00c6735a),
        Color::from_u32(0x007c8fa5),
        Color::from_u32(0x00a5789e),
        Color::from_u32(0x0080cdcb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0096432A);
}
