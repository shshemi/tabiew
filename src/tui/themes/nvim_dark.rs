
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NvimDark;

impl SixColorsTwoRowsStyler for NvimDark {
    const BACKGROUND: Color = Color::from_u32(0x0014161b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0034363B);
    const FOREGROUND: Color = Color::from_u32(0x00e0e2ea);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0007080d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffc0b9),
        Color::from_u32(0x00b3f6c0),
        Color::from_u32(0x00fce094),
        Color::from_u32(0x00a6dbff),
        Color::from_u32(0x00ffcaff),
        Color::from_u32(0x008cf8f7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ffc0b9),
        Color::from_u32(0x00b3f6c0),
        Color::from_u32(0x00fce094),
        Color::from_u32(0x00a6dbff),
        Color::from_u32(0x00ffcaff),
        Color::from_u32(0x008cf8f7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1C21), Color::from_u32(0x00202227)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007B7E84);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9089);

    fn id(&self) -> &str {
        "nvim_dark"
    }

    fn title(&self) -> &str {
        "NvimDark"
    }
}
