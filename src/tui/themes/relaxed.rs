use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Relaxed;

impl SixColorsTwoRowsStyler for Relaxed {
    const BACKGROUND: Color = Color::from_u32(0x00353a44);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00555A64);
    const FOREGROUND: Color = Color::from_u32(0x00d9d9d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00151515);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bc5653),
        Color::from_u32(0x00a0ac77),
        Color::from_u32(0x00ebc17a),
        Color::from_u32(0x007eaac7),
        Color::from_u32(0x00b06698),
        Color::from_u32(0x00acbbd0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bc5653),
        Color::from_u32(0x00909d63),
        Color::from_u32(0x00ebc17a),
        Color::from_u32(0x006a8799),
        Color::from_u32(0x00b06698),
        Color::from_u32(0x00c9dfff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003B404A), Color::from_u32(0x00414650)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B9B9B9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BB914A);
}
