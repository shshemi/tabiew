
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SleepyHollow;

impl SixColorsTwoRowsStyler for SleepyHollow {
    const BACKGROUND: Color = Color::from_u32(0x00121214);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323234);
    const FOREGROUND: Color = Color::from_u32(0x00af9a91);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00572100);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d9443f),
        Color::from_u32(0x00d6b04e),
        Color::from_u32(0x00f66813),
        Color::from_u32(0x008086ef),
        Color::from_u32(0x00e2c2bb),
        Color::from_u32(0x00a4dce7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ba3934),
        Color::from_u32(0x0091773f),
        Color::from_u32(0x00b55600),
        Color::from_u32(0x005f63b4),
        Color::from_u32(0x00a17c7b),
        Color::from_u32(0x008faea9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0018181A), Color::from_u32(0x001E1E20)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008F7A71);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x008A0904);
}
