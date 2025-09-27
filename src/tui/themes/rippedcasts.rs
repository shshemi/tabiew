
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Rippedcasts;

impl SixColorsTwoRowsStyler for Rippedcasts {
    const BACKGROUND: Color = Color::from_u32(0x002b2b2b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004B4B4B);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eecbad),
        Color::from_u32(0x00bcee68),
        Color::from_u32(0x00e5e500),
        Color::from_u32(0x0086bdc9),
        Color::from_u32(0x00e500e5),
        Color::from_u32(0x008c9bc4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cdaf95),
        Color::from_u32(0x00a8ff60),
        Color::from_u32(0x00bfbb1f),
        Color::from_u32(0x0075a5b0),
        Color::from_u32(0x00ff73fd),
        Color::from_u32(0x005a647e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00313131), Color::from_u32(0x00373737)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005F5F5F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF43CD);

    fn id(&self) -> &str {
        "rippedcasts"
    }

    fn title(&self) -> &str {
        "Rippedcasts"
    }
}
