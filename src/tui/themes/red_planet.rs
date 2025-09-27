
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RedPlanet;

impl SixColorsTwoRowsStyler for RedPlanet {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00c2b790);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00202020);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b55242),
        Color::from_u32(0x00869985),
        Color::from_u32(0x00ebeb91),
        Color::from_u32(0x0060827e),
        Color::from_u32(0x00de4974),
        Color::from_u32(0x0038add8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x008c3432),
        Color::from_u32(0x00728271),
        Color::from_u32(0x00e8bf6a),
        Color::from_u32(0x0069819e),
        Color::from_u32(0x00896492),
        Color::from_u32(0x005b8390),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A29770);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B88F3A);

    fn id(&self) -> &str {
        "red_planet"
    }

    fn title(&self) -> &str {
        "RedPlanet"
    }
}
