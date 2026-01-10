use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MaterialOcean;

impl SixColorsTwoRowsStyler for MaterialOcean {
    const BACKGROUND: Color = Color::from_u32(0x000f111a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002F313A);
    const FOREGROUND: Color = Color::from_u32(0x008f93a2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00546e7a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5370),
        Color::from_u32(0x00c3e88d),
        Color::from_u32(0x00ffcb6b),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x0089ddff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5370),
        Color::from_u32(0x00c3e88d),
        Color::from_u32(0x00ffcb6b),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x0089ddff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00151720), Color::from_u32(0x001B1D26)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFAC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9B3B);
}
