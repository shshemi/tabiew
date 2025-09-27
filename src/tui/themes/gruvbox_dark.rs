
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GruvboxDark;

impl SixColorsTwoRowsStyler for GruvboxDark {
    const BACKGROUND: Color = Color::from_u32(0x00282828);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484848);
    const FOREGROUND: Color = Color::from_u32(0x00ebdbb2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00282828);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb4934),
        Color::from_u32(0x00b8bb26),
        Color::from_u32(0x00fabd2f),
        Color::from_u32(0x0083a598),
        Color::from_u32(0x00d3869b),
        Color::from_u32(0x008ec07c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc241d),
        Color::from_u32(0x0098971a),
        Color::from_u32(0x00d79921),
        Color::from_u32(0x00458588),
        Color::from_u32(0x00b16286),
        Color::from_u32(0x00689d6a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E2E2E), Color::from_u32(0x00343434)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CBBB92);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A76900);

    fn id(&self) -> &str {
        "gruvbox_dark"
    }

    fn title(&self) -> &str {
        "GruvboxDark"
    }
}
