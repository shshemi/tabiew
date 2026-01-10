use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Sundried;

impl SixColorsTwoRowsStyler for Sundried {
    const BACKGROUND: Color = Color::from_u32(0x001a1818);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3838);
    const FOREGROUND: Color = Color::from_u32(0x00c9c9c9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00302b2a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00aa000c),
        Color::from_u32(0x00128c21),
        Color::from_u32(0x00fc6a21),
        Color::from_u32(0x007999f7),
        Color::from_u32(0x00fd8aa1),
        Color::from_u32(0x00fad484),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a7463d),
        Color::from_u32(0x00587744),
        Color::from_u32(0x009d602a),
        Color::from_u32(0x00485b98),
        Color::from_u32(0x00864651),
        Color::from_u32(0x009c814f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00201E1E), Color::from_u32(0x00262424)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0077160D);
}
