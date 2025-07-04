pub mod command_palette;
pub mod data_frame_field_info;
pub mod data_frame_info;
pub mod data_frame_meta_info;
pub mod data_frame_names;
pub mod data_frame_table;
pub mod error_popup;
pub mod input;
pub mod schema;
pub mod search_bar;
pub mod sheet;
pub mod side_panel;
pub mod status_bar;
pub mod tab;
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
