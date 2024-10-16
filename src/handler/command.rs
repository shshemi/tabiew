use polars::{df, frame::DataFrame};
use std::collections::HashMap;

use crate::{app::AppAction, AppResult};

pub type ParseFn = fn(&str) -> AppResult<AppAction>;
pub type Registery = HashMap<&'static str, ParseFn>;
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
                parser: |query|{
                    Ok(AppAction::SqlQuery(query.to_owned()))
                },
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":q", ":quit"),
                usage: ":q",
                description: "Quit Tabiew",
                parser: |_|{
                    Ok(AppAction::Quit)
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":goto"),
                usage: ":goto <line_index>",
                description: "Jumps to the <line_index> line",
                parser: |line|{
                    Ok(AppAction::TabularGoto(
                        line.parse::<usize>()?.saturating_sub(1),
                    ))
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":goup"),
                usage: ":goup <lines>",
                description: "Jump <lines> line(s) up",
                parser: |lines|{
                    Ok(match lines {
                        "page" => AppAction::TabularGoUpFullPage,
                        "half" => AppAction::TabularGoUpHalfPage,
                        _ => AppAction::TabularGoUp(lines.parse()?),
                    })
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":godown"),
                usage: ":godown <lines>",
                description: "Jump <lines> line(s) down",
                parser: |lines|{
                    Ok(match lines {
                        "page" => AppAction::TabularGoDownFullPage,
                        "half" => AppAction::TabularGoDownHalfPage,
                        _ => AppAction::TabularGoDown(lines.parse()?),
                    })
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":reset"),
                usage: ":reset",
                description: "Reset the original data frame",
                parser: |_|{
                    Ok(AppAction::TabularReset)
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":help"),
                usage: ":help",
                description: "Show help menu",
                parser: |_|{
                    Ok(AppAction::Help)
                },
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":S", ":select"),
                usage: ":select <column_name(s)>",
                description: "Query current data frame for columns/functions",
                parser: |query|{
                    Ok(AppAction::TabularSelect(query.to_owned()))
                },
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":F", ":filter"),
                usage: ":filter <condition(s)>",
                description: "Filter current data frame, keeping rows were the condition(s) match",
                parser: |query|{
                    Ok(AppAction::TabularFilter(query.to_owned()))
                },
            },
            CommandEntry {
                prefix: Prefix::ShortAndLong(":O", ":order"),
                usage: ":order <column(s)_and_order(s)>",
                description: "Sort current data frame by column(s)",
                parser: |query|{
                    Ok(AppAction::TabularOrder(query.to_owned()))
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":schema"),
                usage: ":schema",
                description: "Show loaded data frame(s), their schmea(s), and their path(s)",
                parser: |_|{
                    Ok(AppAction::SqlSchema)
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":rand"),
                usage: ":rand",
                description: "Select a random row from current data frame",
                parser: |_|{
                    Ok(AppAction::TabularGotoRandom)
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":view"),
                usage: ":view (table | sheet | switch)",
                description: "Change tabular's view to table or sheet",
                parser: |query|{
                    Ok(match query {
                        "table" => AppAction::TabularTableView,
                        "sheet" => AppAction::TabularSheetView,
                        "switch" => AppAction::TabularSwitchView,
                        _ => Err("Invalid view")?,
                    })
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":tabn"),
                usage: ":tabn <query>",
                description: "Create a new tab with the query",
                parser: |query|{
                    Ok(AppAction::TabNew(query.to_owned()))
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":tabr"),
                usage: ":tabr <tab_index>",
                description: "Remove the tab at the index",
                parser: |query|{
                    Ok(AppAction::TabRemove(query.parse()?))
                },
            },
            CommandEntry {
                prefix: Prefix::Long(":tab"),
                usage: ":tab <tab_index>",
                description: "Select the tab at the index",
                parser: |query|{
                    Ok(AppAction::TabSelect(query.parse()?))
                },
            },
        ])
    }
}

impl Commands {
    pub fn into_exec(self) -> Registery {
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