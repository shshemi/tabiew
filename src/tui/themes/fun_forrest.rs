
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FunForrest;

impl SixColorsTwoRowsStyler for FunForrest {
    const BACKGROUND: Color = Color::from_u32(0x00251200);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00453220);
    const FOREGROUND: Color = Color::from_u32(0x00dec165);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e55a1c),
        Color::from_u32(0x00bfc65a),
        Color::from_u32(0x00ffcb1b),
        Color::from_u32(0x007cc9cf),
        Color::from_u32(0x00d26349),
        Color::from_u32(0x00e6a96b),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d6262b),
        Color::from_u32(0x00919c00),
        Color::from_u32(0x00be8a13),
        Color::from_u32(0x004699a3),
        Color::from_u32(0x008d4331),
        Color::from_u32(0x00da8213),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002B1806), Color::from_u32(0x00311E0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C53900);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AA5200);

    fn id(&self) -> &str {
        "fun_forrest"
    }

    fn title(&self) -> &str {
        "FunForrest"
    }
}
