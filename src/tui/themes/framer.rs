
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Framer;

impl SixColorsTwoRowsStyler for Framer {
    const BACKGROUND: Color = Color::from_u32(0x00111111);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313131);
    const FOREGROUND: Color = Color::from_u32(0x00777777);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00141414);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8888),
        Color::from_u32(0x00b6f292),
        Color::from_u32(0x00ffd966),
        Color::from_u32(0x0033bbff),
        Color::from_u32(0x00cebbff),
        Color::from_u32(0x00bbecff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5555),
        Color::from_u32(0x0098ec65),
        Color::from_u32(0x00ffcc33),
        Color::from_u32(0x0000aaff),
        Color::from_u32(0x00aa88ff),
        Color::from_u32(0x0088ddff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00171717), Color::from_u32(0x001D1D1D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DCBC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9C03);

    fn id(&self) -> &str {
        "framer"
    }

    fn title(&self) -> &str {
        "Framer"
    }
}
