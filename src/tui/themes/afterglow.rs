
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Afterglow;

impl SixColorsTwoRowsStyler for Afterglow {
    const BACKGROUND: Color = Color::from_u32(0x00212121);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00414141);
    const FOREGROUND: Color = Color::from_u32(0x00d0d0d0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00151515);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ac4142),
        Color::from_u32(0x007e8e50),
        Color::from_u32(0x00e5b567),
        Color::from_u32(0x006c99bb),
        Color::from_u32(0x009f4e85),
        Color::from_u32(0x007dd6cf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ac4142),
        Color::from_u32(0x007e8e50),
        Color::from_u32(0x00e5b567),
        Color::from_u32(0x006c99bb),
        Color::from_u32(0x009f4e85),
        Color::from_u32(0x007dd6cf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00272727), Color::from_u32(0x002D2D2D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B0B0B0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B58537);

    fn id(&self) -> &str {
        "afterglow"
    }

    fn title(&self) -> &str {
        "Afterglow"
    }
}
