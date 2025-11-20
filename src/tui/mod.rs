pub mod component;
pub mod data_frame_table;
pub mod enumerated_list;
pub mod error_popup;
pub mod pane;
pub mod pickers;
pub mod plots;
pub mod popups;
pub mod schema;
pub mod search_bar;
pub mod sheet;
pub mod status_bar;
pub mod tabs;
pub mod terminal;
pub mod themes;
mod utils;
pub mod widgets;

pub use pane::{PaneState, TableType};
pub use terminal::Terminal;

// pub mod themes {
//     pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
// }
