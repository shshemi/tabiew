
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MelangeLight;

impl SixColorsTwoRowsStyler for MelangeLight {
    const BACKGROUND: Color = Color::from_u32(0x00f1f1f1);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0054433a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00e9e1db);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf0021),
        Color::from_u32(0x003a684a),
        Color::from_u32(0x00a06d00),
        Color::from_u32(0x00465aa4),
        Color::from_u32(0x00904180),
        Color::from_u32(0x003d6568),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c77b8b),
        Color::from_u32(0x006e9b72),
        Color::from_u32(0x00bc5c00),
        Color::from_u32(0x007892bd),
        Color::from_u32(0x00be79bb),
        Color::from_u32(0x00739797),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F7F7F7), Color::from_u32(0x00FDFDFD)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0034231A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00974B5B);
}
