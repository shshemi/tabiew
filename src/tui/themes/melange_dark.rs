
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MelangeDark;

impl SixColorsTwoRowsStyler for MelangeDark {
    const BACKGROUND: Color = Color::from_u32(0x00292522);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494542);
    const FOREGROUND: Color = Color::from_u32(0x00ece1d7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0034302c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d47766),
        Color::from_u32(0x0085b695),
        Color::from_u32(0x00ebc06d),
        Color::from_u32(0x00a3a9ce),
        Color::from_u32(0x00cf9bc2),
        Color::from_u32(0x0089b3b6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bd8183),
        Color::from_u32(0x0078997a),
        Color::from_u32(0x00e49b5d),
        Color::from_u32(0x007f91b2),
        Color::from_u32(0x00b380b0),
        Color::from_u32(0x007b9695),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F2B28), Color::from_u32(0x0035312E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CCC1B7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B46B2D);

    fn id(&self) -> &str {
        "melange_dark"
    }

    fn title(&self) -> &str {
        "MelangeDark"
    }
}
