
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RedAlert;

impl SixColorsTwoRowsStyler for RedAlert {
    const BACKGROUND: Color = Color::from_u32(0x00762423);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00964443);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e02553),
        Color::from_u32(0x00aff08c),
        Color::from_u32(0x00dfddb7),
        Color::from_u32(0x0065aaf1),
        Color::from_u32(0x00ddb7df),
        Color::from_u32(0x00b7dfdd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d62e4e),
        Color::from_u32(0x0071be6b),
        Color::from_u32(0x00beb86b),
        Color::from_u32(0x00489bee),
        Color::from_u32(0x00e979d7),
        Color::from_u32(0x006bbeb8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x007C2A29), Color::from_u32(0x0082302F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B949A7);

    fn id(&self) -> &str {
        "red_alert"
    }

    fn title(&self) -> &str {
        "RedAlert"
    }
}
