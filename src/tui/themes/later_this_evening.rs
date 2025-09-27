
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LaterThisEvening;

impl SixColorsTwoRowsStyler for LaterThisEvening {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00959595);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002b2b2b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d3232f),
        Color::from_u32(0x00aabb39),
        Color::from_u32(0x00e5be39),
        Color::from_u32(0x006699d6),
        Color::from_u32(0x00ab53d6),
        Color::from_u32(0x005fc0ae),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d45a60),
        Color::from_u32(0x00afba67),
        Color::from_u32(0x00e5d289),
        Color::from_u32(0x00a0bad6),
        Color::from_u32(0x00c092d6),
        Color::from_u32(0x0091bfb7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00222222);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B5A259);

    fn id(&self) -> &str {
        "later_this_evening"
    }

    fn title(&self) -> &str {
        "LaterThisEvening"
    }
}
