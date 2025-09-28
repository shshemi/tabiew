
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LabFox;

impl SixColorsTwoRowsStyler for LabFox {
    const BACKGROUND: Color = Color::from_u32(0x002e2e2e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004E4E4E);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002e2e2e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6517),
        Color::from_u32(0x0053eaa8),
        Color::from_u32(0x00fca013),
        Color::from_u32(0x00db501f),
        Color::from_u32(0x00441090),
        Color::from_u32(0x007d53e7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc6d26),
        Color::from_u32(0x003eb383),
        Color::from_u32(0x00fca121),
        Color::from_u32(0x00db3b21),
        Color::from_u32(0x00380d75),
        Color::from_u32(0x006e49cb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00343434), Color::from_u32(0x003A3A3A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005F5F5F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC7100);
}
