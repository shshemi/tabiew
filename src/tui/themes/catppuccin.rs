use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Catppuccin;

impl SixColorsTwoRowsStyler for Catppuccin {
    const BACKGROUND: Color = Color::from_u32(0x0011111b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x001e1e2e);
    const FOREGROUND: Color = Color::from_u32(0x00cdd6f4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003e3e4e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181825), Color::from_u32(0x001e1e2e)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00f9e2af);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x0011111b);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d36b98);
}
