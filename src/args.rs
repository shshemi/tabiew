use clap::{Parser, ValueEnum};
use std::{num::NonZero, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path(s) to the file(s) to be opened.", required = false)]
    pub files: Vec<PathBuf>,

    #[arg(long, help = "Paths to be opened and concatenated vertically.",
        num_args = 1..,
        required = false)]
    pub multiparts: Vec<PathBuf>,

    #[arg(short, long, help = "Path to the startup script.", required = false)]
    pub script: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Specifies the input format. By default, the format is selected based on the file extension",
        value_enum
    )]
    pub format: Option<Format>,

    #[arg(long, help = "Sets the key for sqlite (if required)", value_enum)]
    pub sqlite_key: Option<String>,

    #[arg(
        long,
        help = "Specifies if the input does not contain a header row.",
        default_value_t = false
    )]
    pub no_header: bool,

    #[arg(
        long,
        help = "Ignores parsing errors while loading.",
        default_value_t = false
    )]
    pub ignore_errors: bool,

    #[arg(
        long,
        help = "Specifies the method to infer the schema.",
        required = false,
        value_enum,
        default_value_t = InferSchema::Safe,
    )]
    pub infer_schema: InferSchema,

    #[arg(
        long,
        help = "Performs additional processing to parse date and datetime columns",
        default_value_t = false
    )]
    pub infer_datetimes: bool,

    #[arg(
        long,
        help = "Character used as the field separator or delimiter while loading DSV files.",
        required = false,
        default_value_t = ','
    )]
    pub separator: char,

    #[arg(
        long,
        help = "Character used to quote fields while loading DSV files.",
        required = false,
        default_value_t = '"'
    )]
    pub quote_char: char,

    #[arg(
        long,
        help = "A comma-separated list of widths, which specifies the column widths for FWF files.",
        required = false,
        default_value_t = String::default(),
    )]
    pub widths: String,

    #[arg(
        long,
        help = "Specifies the separator length for FWF files.",
        required = false,
        default_value_t = 1_usize
    )]
    pub separator_length: usize,

    #[arg(
        long,
        help = "Sets strict column width restrictions for FWF files.",
        required = false,
        default_value_t = false
    )]
    pub no_flexible_width: bool,

    #[arg(
        long,
        help = "Truncate ragged lines while reading the file.",
        required = false,
        default_value_t = false
    )]
    pub truncate_ragged_lines: bool,

    #[arg(long, help = "Tabiew theme", required = false, value_enum)]
    pub theme: Option<AppTheme>,

    #[arg(
        long,
        help = "Generate a sample theme file in $HOME/.config/tabiew",
        required = false
    )]
    pub generate: Vec<GenerateItem>,

    #[arg(
        long,
        help = "Specifies the types to infer for text-based files.",
        required = false,
        default_value_t = TypeVec(vec![Type::Int, Type::Float]),
    )]
    pub infer_types: TypeVec,

    #[arg(
        long,
        help = "Disables type inference",
        required = false,
        default_value_t = false
    )]
    pub no_type_inference: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Dsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Fwf,
    Sqlite,
    Excel,
}

#[derive(Debug, Clone)]
pub struct TypeVec(Vec<Type>);

impl TypeVec {
    pub fn inner(&self) -> &[Type] {
        &self.0
    }
}

impl std::fmt::Display for TypeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_strings: Vec<String> = self.0.iter().map(|t| t.to_string()).collect();
        write!(f, "{}", type_strings.join(" "))
    }
}

impl std::str::FromStr for TypeVec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(' ')
            .map(|t| t.trim().parse::<Type>())
            .collect::<Result<Vec<_>, _>>()
            .map(TypeVec)
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Type {
    All,
    Int,
    Float,
    Boolean,
    Date,
    Datetime,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::All => write!(f, "all"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::Date => write!(f, "date"),
            Type::Datetime => write!(f, "datetime"),
        }
    }
}

impl std::str::FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(Type::All),
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "boolean" => Ok(Type::Boolean),
            "date" => Ok(Type::Date),
            "datetime" => Ok(Type::Datetime),
            _ => Err(format!("Unknown type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InferSchema {
    No,
    Fast,
    Safe,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AppTheme {
    AardvarkBlue,
    Abernathy,
    Adventure,
    AdventureTime,
    Adwaita,
    AdwaitaDark,
    Afterglow,
    Alabaster,
    AlienBlood,
    Andromeda,
    AppleClassic,
    AppleSystemColors,
    AppleSystemColorsLight,
    Arcoiris,
    Ardoise,
    Argonaut,
    Arthur,
    AtelierSulphurpool,
    Atom,
    AtomOneDark,
    AtomOneLight,
    Aura,
    Aurora,
    Ayu,
    AyuLight,
    AyuMirage,
    BananaBlueberry,
    Batman,
    BelafonteDay,
    BelafonteNight,
    BirdsOfParadise,
    BlackMetal,
    BlackMetalBathory,
    BlackMetalBurzum,
    BlackMetalDarkFuneral,
    BlackMetalGorgoroth,
    BlackMetalImmortal,
    BlackMetalKhold,
    BlackMetalMarduk,
    BlackMetalMayhem,
    BlackMetalNile,
    BlackMetalVenom,
    Blazer,
    BlueBerryPie,
    BlueDolphin,
    BlueMatrix,
    BlulocoDark,
    BlulocoLight,
    Borland,
    Breadog,
    Breeze,
    BrightLights,
    Broadcast,
    Brogrammer,
    BuiltinDark,
    BuiltinLight,
    BuiltinPastelDark,
    BuiltinSolarizedDark,
    BuiltinSolarizedLight,
    BuiltinTangoDark,
    BuiltinTangoLight,
    Calamity,
    Catppuccin,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Cga,
    Chakra,
    Chalk,
    Chalkboard,
    ChallengerDeep,
    Chester,
    Ciapre,
    Citruszest,
    Clrs,
    CobaltNeon,
    CobaltNext,
    CobaltNextDark,
    CobaltNextMinimal,
    CoffeeTheme,
    Config,
    CrayonPonyFish,
    CursorDark,
    CutiePro,
    Cyberdyne,
    Cyberpunk,
    CyberpunkScarletProtocol,
    DarkModern,
    DarkPastel,
    Darkermatrix,
    Darkmatrix,
    Darkside,
    Dayfox,
    Deep,
    Desert,
    Detuned,
    Dimidium,
    DimmedMonokai,
    Django,
    DjangoRebornAgain,
    DjangoSmooth,
    DoomOne,
    DoomPeacock,
    DotGov,
    Dracula,
    Duckbones,
    DuotoneDark,
    Earthsong,
    ElectronHighlighter,
    Elegant,
    Elemental,
    Elementary,
    Embark,
    EmbersDark,
    Encom,
    Espresso,
    EspressoLibre,
    Everblush,
    EverforestDarkHard,
    EverforestLightMed,
    Fahrenheit,
    Fairyfloss,
    FarmhouseDark,
    FarmhouseLight,
    Fideloper,
    FireflyTraditional,
    FirefoxDev,
    Firewatch,
    FishTank,
    Flat,
    Flatland,
    FlexokiDark,
    FlexokiLight,
    Floraverse,
    ForestBlue,
    Framer,
    FrontEndDelight,
    FunForrest,
    Galaxy,
    Galizur,
    GhosttyDefaultStyleDark,
    Github,
    GithubDark,
    GithubDarkColorblind,
    GithubDarkDefault,
    GithubDarkDimmed,
    GithubDarkHighContrast,
    GithubLightColorblind,
    GithubLightDefault,
    GithubLightHighContrast,
    GitlabDark,
    GitlabDarkGrey,
    GitlabLight,
    Glacier,
    Grape,
    Grass,
    GreyGreen,
    GruberDarker,
    GruvboxDark,
    GruvboxDarkHard,
    GruvboxLight,
    GruvboxLightHard,
    GruvboxMaterial,
    Guezwhoz,
    Hacktober,
    Hardcore,
    Harper,
    HavnDaggry,
    HavnSkumring,
    Heeler,
    Highway,
    HipsterGreen,
    Hivacruz,
    Homebrew,
    Hopscotch,
    Horizon,
    HorizonBright,
    Hurtado,
    Hybrid,
    IcGreenPpl,
    IcOrangePpl,
    IcebergDark,
    IcebergLight,
    Idea,
    IdleToes,
    IrBlack,
    IrixConsole,
    IrixTerminal,
    JackieBrown,
    Japanesque,
    Jellybeans,
    JetbrainsDarcula,
    Jubi,
    KanagawaDragon,
    KanagawaWave,
    Kanagawabones,
    Kibble,
    KittyDefault,
    KittyLowContrast,
    Kolorit,
    Konsolas,
    Kurokula,
    LabFox,
    Laser,
    LaterThisEvening,
    Lavandula,
    LightOwl,
    LiquidCarbon,
    LiquidCarbonTransparent,
    LiquidCarbonTransparentInverse,
    Lovelace,
    ManPage,
    Mariana,
    Material,
    MaterialDark,
    MaterialDarker,
    MaterialDesignColors,
    MaterialOcean,
    Mathias,
    Matrix,
    Medallion,
    MelangeDark,
    MelangeLight,
    Mellifluous,
    Mellow,
    Miasma,
    MidnightInMojave,
    Mirage,
    Misterioso,
    Molokai,
    MonaLisa,
    Monokai,
    MonokaiClassic,
    MonokaiPro,
    MonokaiProLight,
    MonokaiProLightSun,
    MonokaiProMachine,
    MonokaiProOctagon,
    MonokaiProRistretto,
    MonokaiProSpectrum,
    MonokaiRemastered,
    MonokaiSoda,
    MonokaiVivid,
    Moonfly,
    NeobonesDark,
    NeobonesLight,
    Neon,
    Neopolitan,
    Neutron,
    NightOwl,
    NightOwlishLight,
    Nightfox,
    Niji,
    NocturnalWinter,
    Nord,
    NordLight,
    NordWave,
    Novel,
    NvimDark,
    NvimLight,
    Obsidian,
    Ocean,
    OceanicMaterial,
    OceanicNext,
    Ollie,
    OneDoubleDark,
    OneDoubleLight,
    OneHalfDark,
    OneHalfLight,
    OperatorMonoDark,
    OvernightSlumber,
    Oxocarbon,
    PaleNightHc,
    Pandora,
    ParaisoDark,
    PaulMillr,
    PencilDark,
    PencilLight,
    Peppermint,
    PhalaGreenDark,
    PiattoLight,
    Pnevma,
    PoppingAndLocking,
    Powershell,
    Primary,
    Pro,
    ProLight,
    PurpleRain,
    Purplepeter,
    Rapture,
    RaycastDark,
    RaycastLight,
    Rebecca,
    RedAlert,
    RedPlanet,
    RedSands,
    Relaxed,
    Retro,
    RetroLegends,
    Rippedcasts,
    RosePine,
    RosePineDawn,
    RosePineMoon,
    Royal,
    Ryuuko,
    Sakura,
    ScarletProtocol,
    SeaShells,
    SeafoamPastel,
    SelenizedDark,
    SelenizedLight,
    SeoulbonesDark,
    SeoulbonesLight,
    Seti,
    ShadesOfPurple,
    Shaman,
    Slate,
    SleepyHollow,
    Smyck,
    Snazzy,
    SnazzySoft,
    SoftServer,
    SolarizedDarcula,
    SolarizedDarkHigherContrast,
    SolarizedDarkPatched,
    SolarizedOsakaNight,
    Sonokai,
    Spacedust,
    Spacegray,
    SpacegrayBright,
    SpacegrayEighties,
    SpacegrayEightiesDull,
    Spiderman,
    Spring,
    Square,
    SquirrelsongDark,
    Srcery,
    Starlight,
    Sublette,
    Subliminal,
    Sugarplum,
    Sundried,
    Symfonic,
    Synthwave,
    SynthwaveAlpha,
    SynthwaveEverything,
    TangoAdapted,
    TangoHalfAdapted,
    Tearout,
    Teerb,
    Terafox,
    Terminal,
    TerminalBasic,
    TerminalBasicDark,
    ThayerBright,
    TheHulk,
    TinaciousDesignDark,
    TinaciousDesignLight,
    TokyoNight,
    Tokyonight,
    TokyonightDay,
    TokyonightMoon,
    TokyonightNight,
    TokyonightStorm,
    Tomorrow,
    TomorrowNight,
    TomorrowNightBlue,
    TomorrowNightBright,
    TomorrowNightBurns,
    TomorrowNightEighties,
    ToyChest,
    Treehouse,
    Twilight,
    Ubuntu,
    UltraDark,
    UltraViolent,
    UnderTheSea,
    Unikitty,
    Urple,
    Vague,
    Vaughn,
    Vercel,
    Vesper,
    VibrantInk,
    Vimbones,
    VioletDark,
    VioletLight,
    Violite,
    WarmNeon,
    Wez,
    Whimsy,
    WildCherry,
    Wilmersdorf,
    Wombat,
    Wryan,
    XcodeDark,
    XcodeDarkHc,
    XcodeLight,
    XcodeLightHc,
    XcodeWwdc,
    Zenbones,
    ZenbonesDark,
    ZenbonesLight,
    Zenburn,
    Zenburned,
    ZenwrittenDark,
    ZenwrittenLight,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum GenerateItem {
    Config,
    Theme,
}

impl InferSchema {
    pub fn to_csv_infer_schema_length(&self) -> Option<usize> {
        match self {
            InferSchema::No => Some(0),
            InferSchema::Fast => Some(128),
            InferSchema::Safe => Some(0),
        }
    }

    pub fn to_json_infer_schema_length(&self) -> Option<NonZero<usize>> {
        match self {
            InferSchema::No => None,
            InferSchema::Fast => Some(NonZero::new(128).unwrap()),
            InferSchema::Safe => None,
        }
    }
}
