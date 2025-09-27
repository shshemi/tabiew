
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Square;

impl SixColorsTwoRowsStyler for Square {
    const BACKGROUND: Color = Color::from_u32(0x001a1a1a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3A3A);
    const FOREGROUND: Color = Color::from_u32(0x00acacab);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00050505);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f99286),
        Color::from_u32(0x00c3f786),
        Color::from_u32(0x00fcfbcc),
        Color::from_u32(0x00b6defb),
        Color::from_u32(0x00ad7fa8),
        Color::from_u32(0x00d7d9fc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e9897c),
        Color::from_u32(0x00b6377d),
        Color::from_u32(0x00ecebbe),
        Color::from_u32(0x00a9cdeb),
        Color::from_u32(0x0075507b),
        Color::from_u32(0x00c9caec),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00202020), Color::from_u32(0x00262626)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DCDBAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BCBB8E);

    fn id(&self) -> &str {
        "square"
    }

    fn title(&self) -> &str {
        "Square"
    }
}
