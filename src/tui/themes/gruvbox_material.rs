
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GruvboxMaterial;

impl SixColorsTwoRowsStyler for GruvboxMaterial {
    const BACKGROUND: Color = Color::from_u32(0x001d2021);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D4041);
    const FOREGROUND: Color = Color::from_u32(0x00d4be98);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00141617);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d3573b),
        Color::from_u32(0x00c1d041),
        Color::from_u32(0x00eecf75),
        Color::from_u32(0x002c86ff),
        Color::from_u32(0x00fd9bc1),
        Color::from_u32(0x0092a5df),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ea6926),
        Color::from_u32(0x00c1d041),
        Color::from_u32(0x00eecf75),
        Color::from_u32(0x006da3ec),
        Color::from_u32(0x00fd9bc1),
        Color::from_u32(0x00fe9d6e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232627), Color::from_u32(0x00292C2D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE6D3E);
}
