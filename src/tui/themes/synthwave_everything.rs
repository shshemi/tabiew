use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SynthwaveEverything;

impl SixColorsTwoRowsStyler for SynthwaveEverything {
    const BACKGROUND: Color = Color::from_u32(0x002a2139);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004A4159);
    const FOREGROUND: Color = Color::from_u32(0x00f0eff1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00fefefe);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f88414),
        Color::from_u32(0x0072f1b8),
        Color::from_u32(0x00fff951),
        Color::from_u32(0x0036f9f6),
        Color::from_u32(0x00e1acff),
        Color::from_u32(0x00f92aad),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f97e72),
        Color::from_u32(0x0072f1b8),
        Color::from_u32(0x00fede5d),
        Color::from_u32(0x006d77b3),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x00f772e0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0030273F), Color::from_u32(0x00362D45)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0052D198);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CEAE2D);
}
