
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Darkermatrix;

impl SixColorsTwoRowsStyler for Darkermatrix {
    const BACKGROUND: Color = Color::from_u32(0x00070c0e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00272C2E);
    const FOREGROUND: Color = Color::from_u32(0x0028380d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00091013);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0000381d),
        Color::from_u32(0x0090d762),
        Color::from_u32(0x00e2e500),
        Color::from_u32(0x0000ff87),
        Color::from_u32(0x00412a4d),
        Color::from_u32(0x00176c73),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00002e18),
        Color::from_u32(0x006fa64c),
        Color::from_u32(0x00595900),
        Color::from_u32(0x0000cb6b),
        Color::from_u32(0x00412a4d),
        Color::from_u32(0x00125459),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000D1214), Color::from_u32(0x0013181A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00171A06);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x003F761C);

    fn id(&self) -> &str {
        "darkermatrix"
    }

    fn title(&self) -> &str {
        "Darkermatrix"
    }
}
