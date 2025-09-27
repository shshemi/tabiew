
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Purplepeter;

impl SixColorsTwoRowsStyler for Purplepeter {
    const BACKGROUND: Color = Color::from_u32(0x002a1a4a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004A3A6A);
    const FOREGROUND: Color = Color::from_u32(0x00ece7fa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000a0520);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f99f92),
        Color::from_u32(0x00b4be8f),
        Color::from_u32(0x00f2e9bf),
        Color::from_u32(0x0079daed),
        Color::from_u32(0x00ba91d4),
        Color::from_u32(0x00a0a0d6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff796d),
        Color::from_u32(0x0099b481),
        Color::from_u32(0x00efdfac),
        Color::from_u32(0x0066d9ef),
        Color::from_u32(0x00e78fcd),
        Color::from_u32(0x00ba8cff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00302050), Color::from_u32(0x00362656)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF493D);

    fn id(&self) -> &str {
        "purplepeter"
    }

    fn title(&self) -> &str {
        "Purplepeter"
    }
}
