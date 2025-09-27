
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FishTank;

impl SixColorsTwoRowsStyler for FishTank {
    const BACKGROUND: Color = Color::from_u32(0x00232537);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00434557);
    const FOREGROUND: Color = Color::from_u32(0x00ecf0fe);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0003073c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00da4b8a),
        Color::from_u32(0x00dbffa9),
        Color::from_u32(0x00fee6a9),
        Color::from_u32(0x00b2befa),
        Color::from_u32(0x00fda5cd),
        Color::from_u32(0x00a5bd86),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c6004a),
        Color::from_u32(0x00acf157),
        Color::from_u32(0x00fecd5e),
        Color::from_u32(0x00525fb8),
        Color::from_u32(0x00986f82),
        Color::from_u32(0x00968763),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292B3D), Color::from_u32(0x002F3143)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DEAD3E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE9D2E);

    fn id(&self) -> &str {
        "fish_tank"
    }

    fn title(&self) -> &str {
        "FishTank"
    }
}
