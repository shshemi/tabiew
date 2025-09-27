
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DotGov;

impl SixColorsTwoRowsStyler for DotGov {
    const BACKGROUND: Color = Color::from_u32(0x00262c35);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00464C55);
    const FOREGROUND: Color = Color::from_u32(0x00ebebeb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191919);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf091d),
        Color::from_u32(0x003d9751),
        Color::from_u32(0x00f6bb34),
        Color::from_u32(0x0017b2e0),
        Color::from_u32(0x007830b0),
        Color::from_u32(0x008bd2ed),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bf091d),
        Color::from_u32(0x003d9751),
        Color::from_u32(0x00f6bb34),
        Color::from_u32(0x0017b2e0),
        Color::from_u32(0x007830b0),
        Color::from_u32(0x008bd2ed),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002C323B), Color::from_u32(0x00323841)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B9000F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C68B04);

    fn id(&self) -> &str {
        "dot_gov"
    }

    fn title(&self) -> &str {
        "DotGov"
    }
}
