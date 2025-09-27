
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Everblush;

impl SixColorsTwoRowsStyler for Everblush {
    const BACKGROUND: Color = Color::from_u32(0x00141b1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343B3E);
    const FOREGROUND: Color = Color::from_u32(0x00dadada);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00232a2d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef7e7e),
        Color::from_u32(0x0096d988),
        Color::from_u32(0x00f4d67a),
        Color::from_u32(0x0071baf2),
        Color::from_u32(0x00ce89df),
        Color::from_u32(0x0067cbe7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e57474),
        Color::from_u32(0x008ccf7e),
        Color::from_u32(0x00e5c76b),
        Color::from_u32(0x0067b0e8),
        Color::from_u32(0x00c47fd5),
        Color::from_u32(0x006cbfbf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A2124), Color::from_u32(0x0020272A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BABABA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B5973B);

    fn id(&self) -> &str {
        "everblush"
    }

    fn title(&self) -> &str {
        "Everblush"
    }
}
