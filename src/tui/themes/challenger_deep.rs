
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ChallengerDeep;

impl SixColorsTwoRowsStyler for ChallengerDeep {
    const BACKGROUND: Color = Color::from_u32(0x001e1c31);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3C51);
    const FOREGROUND: Color = Color::from_u32(0x00cbe1e7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00141228);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8080),
        Color::from_u32(0x0095ffa4),
        Color::from_u32(0x00ffe9aa),
        Color::from_u32(0x0091ddff),
        Color::from_u32(0x00c991e1),
        Color::from_u32(0x00aaffe4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5458),
        Color::from_u32(0x0062d196),
        Color::from_u32(0x00ffb378),
        Color::from_u32(0x0065b2ff),
        Color::from_u32(0x00906cff),
        Color::from_u32(0x0063f2f1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242237), Color::from_u32(0x002A283D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBDCDC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF2428);

    fn id(&self) -> &str {
        "challenger_deep"
    }

    fn title(&self) -> &str {
        "ChallengerDeep"
    }
}
