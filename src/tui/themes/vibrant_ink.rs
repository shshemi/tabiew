
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VibrantInk;

impl SixColorsTwoRowsStyler for VibrantInk {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00878787);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x0000ff00),
        Color::from_u32(0x00ffff00),
        Color::from_u32(0x000000ff),
        Color::from_u32(0x00ff00ff),
        Color::from_u32(0x0000ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6600),
        Color::from_u32(0x00ccff04),
        Color::from_u32(0x00ffcc00),
        Color::from_u32(0x0044b4cc),
        Color::from_u32(0x009933cc),
        Color::from_u32(0x0044b4cc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3600);

    fn id(&self) -> &str {
        "vibrant_ink"
    }

    fn title(&self) -> &str {
        "VibrantInk"
    }
}
