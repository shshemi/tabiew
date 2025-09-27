
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct EspressoLibre;

impl SixColorsTwoRowsStyler for EspressoLibre {
    const BACKGROUND: Color = Color::from_u32(0x002a211c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004A413C);
    const FOREGROUND: Color = Color::from_u32(0x00b8a898);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef2929),
        Color::from_u32(0x009aff87),
        Color::from_u32(0x00fffb5c),
        Color::from_u32(0x0043a8ed),
        Color::from_u32(0x00ff818a),
        Color::from_u32(0x0034e2e2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc0000),
        Color::from_u32(0x001a921c),
        Color::from_u32(0x00f0e53a),
        Color::from_u32(0x000066ff),
        Color::from_u32(0x00c5656b),
        Color::from_u32(0x0006989a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00302722), Color::from_u32(0x00362D28)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C0B50A);

    fn id(&self) -> &str {
        "espresso_libre"
    }

    fn title(&self) -> &str {
        "EspressoLibre"
    }
}
