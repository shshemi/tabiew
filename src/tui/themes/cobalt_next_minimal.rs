
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CobaltNextMinimal;

impl SixColorsTwoRowsStyler for CobaltNextMinimal {
    const BACKGROUND: Color = Color::from_u32(0x000b1c24);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002B3C44);
    const FOREGROUND: Color = Color::from_u32(0x00d7deea);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00323d47);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e47e8b),
        Color::from_u32(0x00baddbb),
        Color::from_u32(0x00ffdc91),
        Color::from_u32(0x007ac0eb),
        Color::from_u32(0x00f3ccef),
        Color::from_u32(0x0084e4e3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff657a),
        Color::from_u32(0x008cc98f),
        Color::from_u32(0x00ffc64c),
        Color::from_u32(0x00409dd4),
        Color::from_u32(0x00cba3c7),
        Color::from_u32(0x0037b5b4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0011222A), Color::from_u32(0x00172830)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00179594);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF961C);

    fn id(&self) -> &str {
        "cobalt_next_minimal"
    }

    fn title(&self) -> &str {
        "CobaltNextMinimal"
    }
}
