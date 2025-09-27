
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Slate;

impl SixColorsTwoRowsStyler for Slate {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x0035b1d2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00222222);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffcdd9),
        Color::from_u32(0x00beffa8),
        Color::from_u32(0x00d0ccca),
        Color::from_u32(0x007ab0d2),
        Color::from_u32(0x00c5a7d9),
        Color::from_u32(0x008cdfe0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e2a8bf),
        Color::from_u32(0x0081d778),
        Color::from_u32(0x00c4c9c0),
        Color::from_u32(0x00264b49),
        Color::from_u32(0x00a481d3),
        Color::from_u32(0x0015ab9c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0067B3A4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B2788F);

    fn id(&self) -> &str {
        "slate"
    }

    fn title(&self) -> &str {
        "Slate"
    }
}
