use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ScarletProtocol;

impl SixColorsTwoRowsStyler for ScarletProtocol {
    const BACKGROUND: Color = Color::from_u32(0x001c153d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C355D);
    const FOREGROUND: Color = Color::from_u32(0x00e41951);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00101116);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6e67),
        Color::from_u32(0x005ffa68),
        Color::from_u32(0x00fffc67),
        Color::from_u32(0x006871ff),
        Color::from_u32(0x00bd35ec),
        Color::from_u32(0x0060fdff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0051),
        Color::from_u32(0x0000dc84),
        Color::from_u32(0x00faf945),
        Color::from_u32(0x000271b6),
        Color::from_u32(0x00ca30c7),
        Color::from_u32(0x0000c5c7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00221B43), Color::from_u32(0x00282149)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0056DF7F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0021);
}
