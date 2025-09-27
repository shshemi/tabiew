
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AtelierSulphurpool;

impl SixColorsTwoRowsStyler for AtelierSulphurpool {
    const BACKGROUND: Color = Color::from_u32(0x00202746);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00404766);
    const FOREGROUND: Color = Color::from_u32(0x00979db4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00202746);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c76b29),
        Color::from_u32(0x00293256),
        Color::from_u32(0x005e6687),
        Color::from_u32(0x00898ea4),
        Color::from_u32(0x00dfe2f1),
        Color::from_u32(0x009c637a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94922),
        Color::from_u32(0x00ac9739),
        Color::from_u32(0x00c08b30),
        Color::from_u32(0x003d8fd1),
        Color::from_u32(0x006679cc),
        Color::from_u32(0x0022a2c9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262D4C), Color::from_u32(0x002C3352)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00777D94);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00991900);

    fn id(&self) -> &str {
        "atelier_sulphurpool"
    }

    fn title(&self) -> &str {
        "AtelierSulphurpool"
    }
}
