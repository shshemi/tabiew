
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlulocoLight;

impl SixColorsTwoRowsStyler for BlulocoLight {
    const BACKGROUND: Color = Color::from_u32(0x00f9f9f9);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00373a41);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00373a41);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6480),
        Color::from_u32(0x003cbc66),
        Color::from_u32(0x00c5a332),
        Color::from_u32(0x000099e1),
        Color::from_u32(0x00ce33c0),
        Color::from_u32(0x006d93bb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d52753),
        Color::from_u32(0x0023974a),
        Color::from_u32(0x00df631c),
        Color::from_u32(0x00275fe4),
        Color::from_u32(0x00823ff1),
        Color::from_u32(0x0027618d),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D30739);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AF3300);

    fn id(&self) -> &str {
        "bluloco_light"
    }

    fn title(&self) -> &str {
        "BlulocoLight"
    }
}
