
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Spacedust;

impl SixColorsTwoRowsStyler for Spacedust {
    const BACKGROUND: Color = Color::from_u32(0x000a1e24);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002A3E44);
    const FOREGROUND: Color = Color::from_u32(0x00ecf0c1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x006e5346);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8a3a),
        Color::from_u32(0x00aecab8),
        Color::from_u32(0x00ffc878),
        Color::from_u32(0x0067a0ce),
        Color::from_u32(0x00ff8a3a),
        Color::from_u32(0x0083a7b4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e35b00),
        Color::from_u32(0x005cab96),
        Color::from_u32(0x00e3cd7b),
        Color::from_u32(0x000f548b),
        Color::from_u32(0x00e35b00),
        Color::from_u32(0x0006afc7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0010242A), Color::from_u32(0x00162A30)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B32B00);

    fn id(&self) -> &str {
        "spacedust"
    }

    fn title(&self) -> &str {
        "Spacedust"
    }
}
