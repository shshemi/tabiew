pub mod command_pallete;
pub mod data_frame_table;
pub mod error_popup;
pub mod field_info_table;
pub mod input;
pub mod schema;
pub mod search_bar;
pub mod sheet;
pub mod side_panel;
pub mod status_bar;
pub mod tab;
pub mod table_info_table;
pub mod table_names_table;
pub mod tabular;
pub mod terminal;
pub mod theme;
mod utils;

pub use tabular::{TableType, TabularState};
pub use terminal::Terminal;
pub use theme::Styler;

pub mod themes {
    pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
}
