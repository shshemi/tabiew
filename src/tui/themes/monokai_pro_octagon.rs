
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProOctagon;

impl SixColorsTwoRowsStyler for MonokaiProOctagon {
    const BACKGROUND: Color = Color::from_u32(0x00282a3a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484A5A);
    const FOREGROUND: Color = Color::from_u32(0x00eaf2f1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00282a3a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff657a),
        Color::from_u32(0x00bad761),
        Color::from_u32(0x00ffd76d),
        Color::from_u32(0x00ff9b5e),
        Color::from_u32(0x00c39ac9),
        Color::from_u32(0x009cd1bb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff657a),
        Color::from_u32(0x00bad761),
        Color::from_u32(0x00ffd76d),
        Color::from_u32(0x00ff9b5e),
        Color::from_u32(0x00c39ac9),
        Color::from_u32(0x009cd1bb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E3040), Color::from_u32(0x00343646)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0092999D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF6B2E);

    fn id(&self) -> &str {
        "monokai_pro_octagon"
    }

    fn title(&self) -> &str {
        "MonokaiProOctagon"
    }
}
