
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TokyonightDay;

impl SixColorsTwoRowsStyler for TokyonightDay {
    const BACKGROUND: Color = Color::from_u32(0x00e1e2e7);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x003760bf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00e9e9ed);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f52a65),
        Color::from_u32(0x00587539),
        Color::from_u32(0x008c6c3e),
        Color::from_u32(0x002e7de9),
        Color::from_u32(0x009854f1),
        Color::from_u32(0x00007197),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f52a65),
        Color::from_u32(0x00587539),
        Color::from_u32(0x008c6c3e),
        Color::from_u32(0x002e7de9),
        Color::from_u32(0x009854f1),
        Color::from_u32(0x00007197),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00E7E8ED), Color::from_u32(0x00EDEEF3)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0017409F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C50035);

    fn id(&self) -> &str {
        "tokyo_night_day"
    }

    fn title(&self) -> &str {
        "TokyonightDay"
    }
}
