
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GithubDark;

impl SixColorsTwoRowsStyler for GithubDark {
    const BACKGROUND: Color = Color::from_u32(0x00101216);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00303236);
    const FOREGROUND: Color = Color::from_u32(0x008b949e);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f78166),
        Color::from_u32(0x0056d364),
        Color::from_u32(0x00e3b341),
        Color::from_u32(0x006ca4f8),
        Color::from_u32(0x00db61a2),
        Color::from_u32(0x002b7489),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f78166),
        Color::from_u32(0x0056d364),
        Color::from_u32(0x00e3b341),
        Color::from_u32(0x006ca4f8),
        Color::from_u32(0x00db61a2),
        Color::from_u32(0x002b7489),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0016181C), Color::from_u32(0x001C1E22)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A9B1B9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C75136);

    fn id(&self) -> &str {
        "git_hub_dark"
    }

    fn title(&self) -> &str {
        "GithubDark"
    }
}
