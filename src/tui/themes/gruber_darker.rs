
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GruberDarker;

impl SixColorsTwoRowsStyler for GruberDarker {
    const BACKGROUND: Color = Color::from_u32(0x00181818);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00383838);
    const FOREGROUND: Color = Color::from_u32(0x00e4e4e4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00181818);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3851),
        Color::from_u32(0x0042dc00),
        Color::from_u32(0x00ffdb00),
        Color::from_u32(0x0092a7cb),
        Color::from_u32(0x00afafda),
        Color::from_u32(0x0090aa9e),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0a36),
        Color::from_u32(0x0042dc00),
        Color::from_u32(0x00ffdb00),
        Color::from_u32(0x0092a7cb),
        Color::from_u32(0x00a095cb),
        Color::from_u32(0x0090aa9e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1E1E), Color::from_u32(0x00242424)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFBB00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFAB00);

    fn id(&self) -> &str {
        "gruber_darker"
    }

    fn title(&self) -> &str {
        "GruberDarker"
    }
}
