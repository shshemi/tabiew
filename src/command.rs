use crate::app::{AppAction, AppResult};
use polars::{df, frame::DataFrame};
use std::{collections::HashMap, error::Error};

pub type ParseFn = fn(&str) -> Result<AppAction, Box<dyn Error>>;
pub type CommandRegistery = HashMap<&'static str, ParseFn>;
pub enum Prefix {
    Long(&'static str),
    ShortAndLong(&'static str, &'static str),
}

impl Prefix {
    fn short(&self) -> Option<&'static str> {
        match self {
            Prefix::ShortAndLong(short, _) => Some(short),
            _ => None,
        }
    }

    fn long(&self) -> Option<&'static str> {
        match self {
            Prefix::Long(long) => Some(long),
            Prefix::ShortAndLong(_, long) => Some(long),
        }
    }
}

struct CommandEntry {
    prefix: Prefix,
    usage: &'static str,
    description: &'static str,
    parser: ParseFn,
}

pub struct Commands(Vec<CommandEntry>);

impl Default for Commands {
    fn default() -> Self {
        Self(vec![
            CommandEntry {
                prefix: Prefix::ShortAndLong(":Q", ":query"),
                usage: ":Q <query>",
                description:
                    "Query the data in Structured Query Language(SQL). The table name is the file name without extension",
                parser: command_query,
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":q", ":quit"),
                usage: ":q",
                description: "Quit Tabiew",
                parser: command_quit,
            },
            CommandEntry {
                prefix: Prefix::Long(":goto"),
                usage: ":goto <line_index>",
                description: "Jumps to the <line_index> line",
                parser: command_goto,
            },
            CommandEntry {
                prefix: Prefix::Long(":goup"),
                usage: ":goup <lines>",
                description: "Jump <lines> line(s) up",
                parser: command_select_up,
            },
            CommandEntry {
                prefix: Prefix::Long(":godown"),
                usage: ":godown <lines>",
                description: "Jump <lines> line(s) down",
                parser: command_select_down,
            },
            CommandEntry {
                prefix: Prefix::Long(":reset"),
                usage: ":reset",
                description: "Reset the original data frame",
                parser: command_reset,
            },
            CommandEntry {
                prefix: Prefix::Long(":help"),
                usage: ":help",
                description: "Show help menu",
                parser: command_help,
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":S", ":select"),
                usage: ":select <column_name(s)>",
                description: "Query current data frame for columns/functions",
                parser: command_select,
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":F", ":filter"),
                usage: ":filter <condition(s)>",
                description: "Filter current data frame, keeping rows were the condition(s) match",
                parser: command_filter,
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":O", ":order"),
                usage: ":order <column(s)_and_order(s)>",
                description: "Sort current data frame by column(s)",
                parser: command_order,
            },
            CommandEntry {
                prefix: Prefix::Long(":tables"),
                usage: ":tables",
                description: "Show loaded data frame(s) alongside their path(s)",
                parser: command_tables,
            },
            CommandEntry {
                prefix: Prefix::Long(":rand"),
                usage: ":rand",
                description: "Select a random row from current data frame",
                parser: command_select_random_row,
            },
            CommandEntry {
                prefix: Prefix::Long(":view"),
                usage: ":view (table | sheet | switch)",
                description: "Change tabular's view to table or sheet",
                parser: command_change_view,
            },
            CommandEntry {
                prefix: Prefix::Long(":tabn"),
                usage: ":tabn <query>",
                description: "Create a new tab with the query",
                parser: command_new_tab,
            },
            CommandEntry {
                prefix: Prefix::Long(":tabr"),
                usage: ":tabr <tab_index>",
                description: "Remove the tab at the index",
                parser: command_remove_tab,
            },
            CommandEntry {
                prefix: Prefix::Long(":tab"),
                usage: ":tab <tab_index>",
                description: "Select the tab at the index",
                parser: command_select_tab,
            },
        ])
    }
}

impl Commands {
    pub fn into_exec(self) -> CommandRegistery {
        self.0
            .into_iter()
            .flat_map(|cmd| {
                match cmd.prefix {
                    Prefix::Long(long) => vec![(long, cmd.parser)],
                    Prefix::ShortAndLong(short, long) => {
                        vec![(short, cmd.parser), (long, cmd.parser)]
                    }
                }
                .into_iter()
            })
            .collect()
    }

    pub fn into_data_frame(self) -> DataFrame {
        let len = self.0.len();
        let (short, long, usage, description) = self.0.into_iter().fold(
            (
                Vec::<&'static str>::with_capacity(len),
                Vec::<&'static str>::with_capacity(len),
                Vec::<&'static str>::with_capacity(len),
                Vec::<&'static str>::with_capacity(len),
            ),
            |(mut v1, mut v2, mut v3, mut v4), cmd| {
                v1.push(cmd.prefix.short().unwrap_or("-"));
                v2.push(cmd.prefix.long().unwrap_or("-"));
                v3.push(cmd.usage);
                v4.push(cmd.description);
                (v1, v2, v3, v4)
            },
        );
        df! {
            "Command" => long,
            "Short Form" => short,
            "Usage" => usage,
            "Description" => description,
        }
        .unwrap()
    }
}

fn command_query(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::SqlQuery(query.to_owned()))
}

fn command_quit(_query: &str) -> AppResult<AppAction> {
    Ok(AppAction::Quit)
}

fn command_goto(line: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularGoto(
        line.parse::<usize>()?.saturating_sub(1),
    ))
}

fn command_select_up(lines: &str) -> AppResult<AppAction> {
    Ok(match lines {
        "page" => AppAction::TabularGoUpFullPage,
        "half" => AppAction::TabularGoUpHalfPage,
        _ => AppAction::TabularGoUp(lines.parse()?),
    })
}

fn command_select_down(lines: &str) -> AppResult<AppAction> {
    Ok(match lines {
        "page" => AppAction::TabularGoDownFullPage,
        "half" => AppAction::TabularGoDownHalfPage,
        _ => AppAction::TabularGoDown(lines.parse()?),
    })
}

fn command_reset(_: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularReset)
}

fn command_help(_: &str) -> AppResult<AppAction> {
    Ok(AppAction::Help)
}

fn command_select(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularSelect(query.to_owned()))
}

fn command_filter(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularFilter(query.to_owned()))
}

fn command_order(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularOrder(query.to_owned()))
}

fn command_tables(_query: &str) -> AppResult<AppAction> {
    Ok(AppAction::SqlBackendTable)
}

fn command_change_view(query: &str) -> AppResult<AppAction> {
    Ok(match query {
        "table" => AppAction::TabularTableView,
        "sheet" => AppAction::TabularSheetView,
        "switch" => AppAction::TabularSwitchView,
        _ => Err("Invalid view")?,
    })
}

fn command_select_random_row(_query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabularGotoRandom)
}

fn command_new_tab(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabNew(query.to_owned()))

}

fn command_remove_tab(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabRemove(query.parse()?))
}

fn command_select_tab(query: &str) -> AppResult<AppAction> {
    Ok(AppAction::TabSelect(query.parse()?))
}