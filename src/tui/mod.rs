pub mod component;
pub mod error_popup;
pub mod pane;
pub mod pickers;
pub mod plots;
pub mod popups;
pub mod schema;
pub mod search_bar;
pub mod sheet;
pub mod status_bar;
pub mod tab_switcher;
pub mod table;
pub mod tabs;
pub mod terminal;
pub mod themes;
mod utils;
pub mod widgets;

pub use pane::{Pane, TableType};
pub use terminal::Terminal;

// pub mod themes {
//     pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
// }
