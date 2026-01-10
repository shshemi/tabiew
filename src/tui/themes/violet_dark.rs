use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VioletDark;

impl SixColorsTwoRowsStyler for VioletDark {
    const BACKGROUND: Color = Color::from_u32(0x001c1d1f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3D3F);
    const FOREGROUND: Color = Color::from_u32(0x00708284);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0056595c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bd3613),
        Color::from_u32(0x00738a04),
        Color::from_u32(0x00a57705),
        Color::from_u32(0x002176c7),
        Color::from_u32(0x00c61c6f),
        Color::from_u32(0x00259286),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94c22),
        Color::from_u32(0x0085981c),
        Color::from_u32(0x00b4881d),
        Color::from_u32(0x002e8bce),
        Color::from_u32(0x00d13a82),
        Color::from_u32(0x0032a198),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00222325), Color::from_u32(0x0028292B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A10A52);
}
