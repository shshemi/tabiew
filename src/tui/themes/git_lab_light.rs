
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GitlabLight;

impl SixColorsTwoRowsStyler for GitlabLight {
    const BACKGROUND: Color = Color::from_u32(0x00fafaff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00303030);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00303030);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00a31700),
        Color::from_u32(0x000a7f3d),
        Color::from_u32(0x00af551d),
        Color::from_u32(0x00006cd8),
        Color::from_u32(0x00583cac),
        Color::from_u32(0x0000798a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a31700),
        Color::from_u32(0x000a7f3d),
        Color::from_u32(0x00af551d),
        Color::from_u32(0x00006cd8),
        Color::from_u32(0x00583cac),
        Color::from_u32(0x0000798a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00101010);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x007F2500);
}
