
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Neon;

impl SixColorsTwoRowsStyler for Neon {
    const BACKGROUND: Color = Color::from_u32(0x0014161a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0034363A);
    const FOREGROUND: Color = Color::from_u32(0x0000fffc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5a5a),
        Color::from_u32(0x0075ff88),
        Color::from_u32(0x00fffd96),
        Color::from_u32(0x003c40cb),
        Color::from_u32(0x00f15be5),
        Color::from_u32(0x0088fffe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3045),
        Color::from_u32(0x005ffa74),
        Color::from_u32(0x00fffc7e),
        Color::from_u32(0x000208cb),
        Color::from_u32(0x00f924e7),
        Color::from_u32(0x0000fffc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1C20), Color::from_u32(0x00202226)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0015);
}
