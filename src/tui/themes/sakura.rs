use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Sakura;

impl SixColorsTwoRowsStyler for Sakura {
    const BACKGROUND: Color = Color::from_u32(0x0018131e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0038333E);
    const FOREGROUND: Color = Color::from_u32(0x00dd7bdc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f41d99),
        Color::from_u32(0x0022e529),
        Color::from_u32(0x00f59574),
        Color::from_u32(0x009892f1),
        Color::from_u32(0x00e90cdd),
        Color::from_u32(0x00eeeeee),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d52370),
        Color::from_u32(0x0041af1a),
        Color::from_u32(0x00bc7053),
        Color::from_u32(0x006964ab),
        Color::from_u32(0x00c71fbf),
        Color::from_u32(0x00939393),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1924), Color::from_u32(0x00241F2A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF45DD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A50040);
}
