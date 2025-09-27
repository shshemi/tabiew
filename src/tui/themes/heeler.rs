
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Heeler;

impl SixColorsTwoRowsStyler for Heeler {
    const BACKGROUND: Color = Color::from_u32(0x00211f46);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00413F66);
    const FOREGROUND: Color = Color::from_u32(0x00fdfdfd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e44c2e),
        Color::from_u32(0x00bdd100),
        Color::from_u32(0x00f4ce65),
        Color::from_u32(0x000088ff),
        Color::from_u32(0x00ff95c2),
        Color::from_u32(0x008da6e4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e44c2e),
        Color::from_u32(0x00bdd100),
        Color::from_u32(0x00f4ce65),
        Color::from_u32(0x005ba5f2),
        Color::from_u32(0x00ff95c2),
        Color::from_u32(0x00ff9763),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0027254C), Color::from_u32(0x002D2B52)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF6733);

    fn id(&self) -> &str {
        "heeler"
    }

    fn title(&self) -> &str {
        "Heeler"
    }
}
