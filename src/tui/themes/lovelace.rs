
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Lovelace;

impl SixColorsTwoRowsStyler for Lovelace {
    const BACKGROUND: Color = Color::from_u32(0x001d1f28);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3F48);
    const FOREGROUND: Color = Color::from_u32(0x00fdfdfd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00282a36);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4971),
        Color::from_u32(0x0018e3c8),
        Color::from_u32(0x00ff8037),
        Color::from_u32(0x00556fff),
        Color::from_u32(0x00b043d1),
        Color::from_u32(0x003fdcee),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f37f97),
        Color::from_u32(0x005adecd),
        Color::from_u32(0x00f2a272),
        Color::from_u32(0x008897f4),
        Color::from_u32(0x00c574dd),
        Color::from_u32(0x0079e6f3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0023252E), Color::from_u32(0x00292B34)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A554BD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C34F67);

    fn id(&self) -> &str {
        "lovelace"
    }

    fn title(&self) -> &str {
        "Lovelace"
    }
}
