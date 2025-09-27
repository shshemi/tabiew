
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MaterialDarker;

impl SixColorsTwoRowsStyler for MaterialDarker {
    const BACKGROUND: Color = Color::from_u32(0x00212121);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00414141);
    const FOREGROUND: Color = Color::from_u32(0x00eeffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00272727), Color::from_u32(0x002D2D2D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9B3B);

    fn id(&self) -> &str {
        "material_darker"
    }

    fn title(&self) -> &str {
        "MaterialDarker"
    }
}
