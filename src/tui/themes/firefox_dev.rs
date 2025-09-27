
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FirefoxDev;

impl SixColorsTwoRowsStyler for FirefoxDev {
    const BACKGROUND: Color = Color::from_u32(0x000e1011);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E3031);
    const FOREGROUND: Color = Color::from_u32(0x007c8fa4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00002831);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e1003f),
        Color::from_u32(0x001d9000),
        Color::from_u32(0x00cd9409),
        Color::from_u32(0x00006fc0),
        Color::from_u32(0x00a200da),
        Color::from_u32(0x00005794),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e63853),
        Color::from_u32(0x005eb83c),
        Color::from_u32(0x00a57706),
        Color::from_u32(0x00359ddf),
        Color::from_u32(0x00d75cff),
        Color::from_u32(0x004b73a2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00141617), Color::from_u32(0x001A1C1D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B60823);

    fn id(&self) -> &str {
        "firefox_dev"
    }

    fn title(&self) -> &str {
        "FirefoxDev"
    }
}
