use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use ratatui::style::{Color, Style, Stylize};
use serde::{Deserialize, Serialize};

use crate::args::AppTheme;

#[derive(Debug, Clone)]
pub struct Theme {
    styler: Option<Arc<dyn Styler + Send + Sync>>,
}

impl Theme {
    pub fn new<S: Styler + Send + Sync + 'static>(theme: S) -> Self {
        Theme {
            styler: Some(Arc::new(theme)),
        }
    }

    fn styler(&self) -> Arc<dyn Styler> {
        self.styler.clone().unwrap_or(Arc::new(Monokai))
    }

    pub fn all() -> &'static [Theme] {
        static ALL: OnceLock<Vec<Theme>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Theme::new(Monokai),
                Theme::new(Argonaut),
                Theme::new(Nord),
                Theme::new(Catppuccin),
                Theme::new(TokyoNight),
                Theme::new(Chakra),
                Theme::new(Terminal),
                Theme::new(Custom::default()),
            ]
        })
    }

    pub const fn default() -> Self {
        Self { styler: None }
    }
}

impl From<AppTheme> for Theme {
    fn from(value: AppTheme) -> Self {
        match value {
            AppTheme::Monokai => Theme::new(Monokai),
            AppTheme::Argonaut => Theme::new(Argonaut),
            AppTheme::Nord => Theme::new(Nord),
            AppTheme::Catppuccin => Theme::new(Catppuccin),
            AppTheme::TokyoNight => Theme::new(TokyoNight),
            AppTheme::Terminal => Theme::new(Terminal),
            AppTheme::Chakra => Theme::new(Chakra),
            AppTheme::Config => Theme::new(Custom::default()),
        }
    }
}

impl Styler for Theme {
    fn table_header(&self) -> Style {
        self.styler().table_header()
    }

    fn row(&self, row: usize) -> Style {
        self.styler().row(row)
    }

    fn highlight(&self) -> Style {
        self.styler().highlight()
    }

    fn header(&self, col: usize) -> Style {
        self.styler().header(col)
    }

    fn tag(&self, col: usize) -> Style {
        self.styler().tag(col)
    }

    fn block(&self) -> Style {
        self.styler().block()
    }

    fn block_tag(&self) -> Style {
        self.styler().block_tag()
    }

    fn text(&self) -> Style {
        self.styler().text()
    }

    fn subtext(&self) -> Style {
        self.styler().subtext()
    }

    fn error(&self) -> Style {
        self.styler().error()
    }

    fn graph(&self, idx: usize) -> Style {
        self.styler().graph(idx)
    }

    fn id(&self) -> &str {
        if let Some(inner) = self.styler.as_ref() {
            inner.id()
        } else {
            "monokai"
        }
    }

    fn title(&self) -> &str {
        if let Some(inner) = self.styler.as_ref() {
            inner.title()
        } else {
            "Monokai"
        }
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.styler().id() == other.styler().id()
    }
}

impl Eq for Theme {}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let theme_str = String::deserialize(deserializer)?;
        match theme_str.as_str() {
            "monokai" => Ok(Theme::new(Monokai)),
            "argonaut" => Ok(Theme::new(Monokai)),
            "terminal" => Ok(Theme::new(Monokai)),
            "nord" => Ok(Theme::new(Monokai)),
            "catppuccin" => Ok(Theme::new(Monokai)),
            "tokyo-night" => Ok(Theme::new(Monokai)),
            "chakra" => Ok(Theme::new(Monokai)),
            "custom" => Ok(Theme::new(Custom::default())),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown theme: {}",
                theme_str
            ))),
        }
    }
}

pub trait Styler: Debug {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
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
    fn graph(&self, idx: usize) -> Style;
}

pub trait SixColorsTwoRowsStyler {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    const BACKGROUND: Color;
    const LIGHT_BACKGROUND: Color;
    const FOREGROUND: Color;
    const DARK_FOREGROUND: Color;

    const COLORS: [Color; 6];
    const DARK_COLORS: [Color; 6];

    const ROW_BACKGROUNDS: [Color; 2];
    const HIGHLIGHT_BACKGROUND: Color;
    const HIGHLIGHT_FOREGROUND: Color;

    const STATUS_BAR_ERROR: Color;
}

impl<T> Styler for T
where
    T: SixColorsTwoRowsStyler + Debug,
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
            .bg(Self::HIGHLIGHT_BACKGROUND)
            .fg(Self::HIGHLIGHT_FOREGROUND)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn block(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHLIGHT_BACKGROUND)
    }

    fn block_tag(&self) -> Style {
        Style::default()
            .bg(Self::HIGHLIGHT_BACKGROUND)
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

    fn graph(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .bold()
    }

    fn id(&self) -> &str {
        self.id()
    }

    fn title(&self) -> &str {
        self.title()
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Monokai;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Argonaut;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Terminal;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Nord;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Catppuccin;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TokyoNight;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Chakra;

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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00c89f2d);
    const HIGHLIGHT_FOREGROUND: Color = Self::BACKGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d02d00);

    fn id(&self) -> &str {
        "monokai"
    }

    fn title(&self) -> &str {
        "Monokai"
    }
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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00204a5b);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00dd0000);

    fn id(&self) -> &str {
        "argonaut"
    }

    fn title(&self) -> &str {
        "Argonaut"
    }
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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBBB7B);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x002E3440);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF616A);

    fn id(&self) -> &str {
        "nord"
    }

    fn title(&self) -> &str {
        "Nord"
    }
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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00f9e2af);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x0011111b);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d36b98);

    fn id(&self) -> &str {
        "catppuccin"
    }

    fn title(&self) -> &str {
        "Catppuccin"
    }
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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ffc777);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x001f2335);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00c53b53);

    fn id(&self) -> &str {
        "tokyo-night"
    }

    fn title(&self) -> &str {
        "Tokyo Night"
    }
}

impl SixColorsTwoRowsStyler for Chakra {
    const BACKGROUND: Color = Color::from_u32(0x00111111);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00191917);
    const FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00a1a1aa);
    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f472b6),
        Color::from_u32(0x00c084fc),
        Color::from_u32(0x0022d3ee),
        Color::from_u32(0x0060a5fa),
        Color::from_u32(0x002dd4bf),
        Color::from_u32(0x004ade80),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec4899),
        Color::from_u32(0x00a855f7),
        Color::from_u32(0x0006b6d4),
        Color::from_u32(0x003b82f6),
        Color::from_u32(0x0014b8a6),
        Color::from_u32(0x0022c55e),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0018181b), Color::from_u32(0x00111111)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ca8a04);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00991919);

    fn id(&self) -> &str {
        "chakra"
    }

    fn title(&self) -> &str {
        "Chakra"
    }
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

    fn graph(&self, _idx: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn id(&self) -> &str {
        "terminal"
    }

    fn title(&self) -> &str {
        "Terminal"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(default)]
pub struct Custom {
    table_header: Style,
    table_headers: Vec<Style>,
    rows: Vec<Style>,
    highlight: Style,
    table_tags: Vec<Style>,
    block: Style,
    block_tag: Style,
    text: Style,
    subtext: Style,
    error: Style,
    chart: Vec<Style>,
}

impl Default for Custom {
    fn default() -> Self {
        Custom {
            table_header: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            table_headers: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
            rows: vec![
                Style::default()
                    .bg(Color::from_u32(0x00383838))
                    .fg(Color::from_u32(0x00ffffff)),
                Style::default()
                    .bg(Color::from_u32(0x00101010))
                    .fg(Color::from_u32(0x00ffffff)),
            ],
            highlight: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            table_tags: vec![
                Style::default()
                    .bg(Color::from_u32(0x00ff0000))
                    .fg(Color::from_u32(0x00000000)),
                Style::default()
                    .bg(Color::from_u32(0x0000ff00))
                    .fg(Color::from_u32(0x00000000)),
            ],
            block: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffff00)),
            block_tag: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            text: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            subtext: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ababab)),
            error: Style::default()
                .bg(Color::from_u32(0x00ff0000))
                .fg(Color::from_u32(0x00ffffff)),
            chart: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
        }
    }
}

impl super::Styler for Custom {
    fn table_header(&self) -> Style {
        self.table_header
    }

    fn header(&self, idx: usize) -> Style {
        self.table_headers[idx % self.table_headers.len()]
    }

    fn row(&self, idx: usize) -> Style {
        self.rows[idx % self.rows.len()]
    }

    fn highlight(&self) -> Style {
        self.highlight
    }

    fn tag(&self, idx: usize) -> Style {
        self.table_tags[idx % self.table_tags.len()]
    }

    fn block(&self) -> Style {
        self.block
    }

    fn block_tag(&self) -> Style {
        self.block_tag
    }

    fn text(&self) -> Style {
        self.text
    }

    fn subtext(&self) -> Style {
        self.subtext
    }

    fn error(&self) -> Style {
        self.error
    }

    fn graph(&self, idx: usize) -> Style {
        self.chart[idx % self.chart.len()]
    }

    fn id(&self) -> &str {
        "custom"
    }

    fn title(&self) -> &str {
        "Custom"
    }
}
