use ratatui::style::Color;

use crate::tui::theme::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Nord;

impl SixColorsTwoRowsStyler for Nord {
    const BACKGROUND: Color = Color::from_u32(0x002E3440);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B4252);
    const FOREGROUND: Color = Color::from_u32(0x00ECEFF4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005B6272);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00BF616A),
        Color::from_u32(0x00D08770),
        Color::from_u32(0x00EBCB8B),
        Color::from_u32(0x00A3BE8C),
        Color::from_u32(0x00B48EAD),
        Color::from_u32(0x005E81AC),
    ];

    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00AF515A),
        Color::from_u32(0x00C07760),
        Color::from_u32(0x00DBBB7B),
        Color::from_u32(0x0093AE7C),
        Color::from_u32(0x00A47E9D),
        Color::from_u32(0x004E719C),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003B4252), Color::from_u32(0x00434C5E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBBB7B);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x002E3440);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF616A);

    fn id(&self) -> &str {
        "nord"
    }

    fn title(&self) -> &str {
        "Nord"
    }
}
