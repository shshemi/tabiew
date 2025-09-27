
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Starlight;

impl SixColorsTwoRowsStyler for Starlight {
    const BACKGROUND: Color = Color::from_u32(0x00242424);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00444444);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00242424);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4d51),
        Color::from_u32(0x0035d450),
        Color::from_u32(0x00e9e836),
        Color::from_u32(0x005dc5f8),
        Color::from_u32(0x00feabf2),
        Color::from_u32(0x0024dfc4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f62b5a),
        Color::from_u32(0x0047b413),
        Color::from_u32(0x00e3c401),
        Color::from_u32(0x0024acd4),
        Color::from_u32(0x00f2affd),
        Color::from_u32(0x0013c299),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A2A2A), Color::from_u32(0x00303030)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C6002A);

    fn id(&self) -> &str {
        "starlight"
    }

    fn title(&self) -> &str {
        "Starlight"
    }
}
