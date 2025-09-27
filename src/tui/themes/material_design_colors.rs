
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MaterialDesignColors;

impl SixColorsTwoRowsStyler for MaterialDesignColors {
    const BACKGROUND: Color = Color::from_u32(0x001d262a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D464A);
    const FOREGROUND: Color = Color::from_u32(0x00e7ebed);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00435b67);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc746d),
        Color::from_u32(0x00adf7be),
        Color::from_u32(0x00fee16c),
        Color::from_u32(0x0070cfff),
        Color::from_u32(0x00fc669b),
        Color::from_u32(0x009affe6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc3841),
        Color::from_u32(0x005cf19e),
        Color::from_u32(0x00fed032),
        Color::from_u32(0x0037b6ff),
        Color::from_u32(0x00fc226e),
        Color::from_u32(0x0059ffd1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232C30), Color::from_u32(0x00293236)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CACACA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CEA002);

    fn id(&self) -> &str {
        "material_design_colors"
    }

    fn title(&self) -> &str {
        "MaterialDesignColors"
    }
}
