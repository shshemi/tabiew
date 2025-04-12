use ratatui::style::{Color, Style, Stylize};

pub trait Styler {
    fn table_header(&self) -> Style;
    fn row(&self, row: usize) -> Style;
    fn highlight(&self) -> Style;
    fn header(&self, col: usize) -> Style;
    fn tag(&self, col: usize) -> Style;
    fn block(&self) -> Style;
    fn block_tag(&self) -> Style;
    fn text(&self) -> Style;
    fn subtext(&self) -> Style;
    fn error(&self) -> Style;
}

pub trait SixColorsTwoRowsStyler {
    const BACKGROUND: Color;
    const LIGHT_BACKGROUND: Color;
    const FOREGROUND: Color;
    const DARK_FOREGROUND: Color;

    const COLORS: [Color; 6];
    const DARK_COLORS: [Color; 6];

    const ROW_BACKGROUNDS: [Color; 2];
    const HIGHTLIGHT_BACKGROUND: Color;
    const HIGHTLIGHT_FOREGROUND: Color;

    const STATUS_BAR_ERROR: Color;
}

impl<T> Styler for T
where
    T: SixColorsTwoRowsStyler,
{
    fn table_header(&self) -> Style {
        Style::default().bg(Self::BACKGROUND)
    }

    fn header(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::COLORS[idx % Self::COLORS.len()])
            .bold()
    }

    fn row(&self, idx: usize) -> Style {
        Style::new()
            .bg(Self::ROW_BACKGROUNDS[idx % Self::ROW_BACKGROUNDS.len()])
            .fg(Self::FOREGROUND)
    }

    fn highlight(&self) -> Style {
        Style::new()
            .bg(Self::HIGHTLIGHT_BACKGROUND)
            .fg(Self::HIGHTLIGHT_FOREGROUND)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn block(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHTLIGHT_BACKGROUND)
    }

    fn block_tag(&self) -> Style {
        Style::default()
            .bg(Self::HIGHTLIGHT_BACKGROUND)
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn text(&self) -> Style {
        Style::default().bg(Self::BACKGROUND).fg(Self::FOREGROUND)
    }

    fn subtext(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::DARK_FOREGROUND)
    }

    fn error(&self) -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_ERROR)
            .fg(Self::FOREGROUND)
    }
}

#[derive(Debug, Default)]
pub struct Monokai;

#[derive(Debug, Default)]
pub struct Argonaut;

#[derive(Debug, Default)]
pub struct Terminal;

#[derive(Debug, Default)]
pub struct Nord;

#[derive(Debug, Default)]
pub struct Catppuccin;

#[derive(Debug, Default)]
pub struct TokyoNight;

impl SixColorsTwoRowsStyler for Monokai {
    const BACKGROUND: Color = Color::from_u32(0x00141115);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003e3b3f);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005e5b5f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6188),
        Color::from_u32(0x00fc9867),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x00a9dc76),
        Color::from_u32(0x0078dce8),
        Color::from_u32(0x00ab9df2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ee4066),
        Color::from_u32(0x00da7645),
        Color::from_u32(0x00ddb644),
        Color::from_u32(0x0087ba54),
        Color::from_u32(0x0056bac6),
        Color::from_u32(0x00897bd0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232024), Color::from_u32(0x001c191d)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00c89f2d);
    const HIGHTLIGHT_FOREGROUND: Color = Self::BACKGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d02d00);
}

impl SixColorsTwoRowsStyler for Argonaut {
    const BACKGROUND: Color = Color::from_u32(0x0001030b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0023252d);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0043454d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0011131b), Color::from_u32(0x0001030b)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00204a5b);
    const HIGHTLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00dd0000);
}

impl SixColorsTwoRowsStyler for Nord {
    const BACKGROUND: Color = Color::from_u32(0x002E3440);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B4252);
    const FOREGROUND: Color = Color::from_u32(0x00ECEFF4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005B6272);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00BF616A),
        Color::from_u32(0x00D08770),
        Color::from_u32(0x00EBCB8B),
        Color::from_u32(0x00A3BE8C),
        Color::from_u32(0x00B48EAD),
        Color::from_u32(0x005E81AC),
    ];

    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00AF515A),
        Color::from_u32(0x00C07760),
        Color::from_u32(0x00DBBB7B),
        Color::from_u32(0x0093AE7C),
        Color::from_u32(0x00A47E9D),
        Color::from_u32(0x004E719C),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003B4252), Color::from_u32(0x00434C5E)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBBB7B);
    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x002E3440);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF616A);
}

impl SixColorsTwoRowsStyler for Catppuccin {
    const BACKGROUND: Color = Color::from_u32(0x0011111b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x001e1e2e);
    const FOREGROUND: Color = Color::from_u32(0x00cdd6f4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003e3e4e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181825), Color::from_u32(0x001e1e2e)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00f9e2af);
    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x0011111b);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d36b98);
}

impl SixColorsTwoRowsStyler for TokyoNight {
    const BACKGROUND: Color = Color::from_u32(0x001f2335);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00292e42);
    const FOREGROUND: Color = Color::from_u32(0x00dfe3f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00494e62);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292e42), Color::from_u32(0x0024283b)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00ffc777);
    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x001f2335);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00c53b53);
}

impl Styler for Terminal {
    fn table_header(&self) -> Style {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    }

    fn row(&self, _row: usize) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn highlight(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Black)
    }

    fn header(&self, _col: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg([
                Color::Red,
                Color::Magenta,
                Color::Blue,
                Color::Cyan,
                Color::Green,
            ][idx % 5])
            .fg(Color::Gray)
    }

    fn block_tag(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Gray)
    }

    fn block(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn text(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn subtext(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::DarkGray)
    }

    fn error(&self) -> Style {
        Style::default().bg(Color::Red).fg(Color::White)
    }
}
