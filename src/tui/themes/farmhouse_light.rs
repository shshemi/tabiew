
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FarmhouseLight;

impl SixColorsTwoRowsStyler for FarmhouseLight {
    const BACKGROUND: Color = Color::from_u32(0x00e8e4e1);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x001d2027);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d2027);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb0009),
        Color::from_u32(0x007ac100),
        Color::from_u32(0x00ea9a00),
        Color::from_u32(0x00006efe),
        Color::from_u32(0x00bf3b7f),
        Color::from_u32(0x0019e062),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x008d0003),
        Color::from_u32(0x003a7d00),
        Color::from_u32(0x00a95600),
        Color::from_u32(0x00092ccd),
        Color::from_u32(0x00820046),
        Color::from_u32(0x00229256),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00EEEAE7), Color::from_u32(0x00F4F0ED)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00004EDE);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00792600);

    fn id(&self) -> &str {
        "farmhouse_light"
    }

    fn title(&self) -> &str {
        "FarmhouseLight"
    }
}
