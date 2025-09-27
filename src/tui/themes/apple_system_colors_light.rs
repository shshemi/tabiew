
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AppleSystemColorsLight;

impl SixColorsTwoRowsStyler for AppleSystemColorsLight {
    const BACKGROUND: Color = Color::from_u32(0x00feffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1a1a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff453a),
        Color::from_u32(0x0032d74b),
        Color::from_u32(0x00ffd60a),
        Color::from_u32(0x000a84ff),
        Color::from_u32(0x00bf5af2),
        Color::from_u32(0x0076d6ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc372e),
        Color::from_u32(0x0026a439),
        Color::from_u32(0x00cdac08),
        Color::from_u32(0x000869cb),
        Color::from_u32(0x009647bf),
        Color::from_u32(0x00479ec2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0078787D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009D7C00);

    fn id(&self) -> &str {
        "apple_system_colors_light"
    }

    fn title(&self) -> &str {
        "AppleSystemColorsLight"
    }
}
