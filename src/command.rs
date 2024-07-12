use std::{collections::HashMap, error::Error, path::PathBuf};

use polars::{df, frame::DataFrame};

use crate::{app::Tabular, sql::SqlBackend};

pub type ExecutionFunction =
    fn(&str, &mut Tabular, &mut SqlBackend, &mut bool) -> Result<(), Box<dyn Error>>;
pub type ExecutionTable = HashMap<&'static str, ExecutionFunction>;
pub enum Prefix {
    Short(&'static str),
    Long(&'static str),
    Both(&'static str, &'static str),
}

impl Prefix {
    fn short(&self) -> Option<&'static str> {
        match self {
            Prefix::Short(short) => Some(short),
            Prefix::Both(short, _) => Some(short),
            _ => None,
        }
    }

    fn long(&self) -> Option<&'static str> {
        match self {
            Prefix::Long(long) => Some(long),
            Prefix::Both(_, long) => Some(long),
            _ => None,
        }
    }
}

pub struct Command {
    prefix: Prefix,
    usage: &'static str,
    description: &'static str,
    function: ExecutionFunction,
}

pub struct CommandList(Vec<Command>);

impl Default for CommandList {
    fn default() -> Self {
        Self(vec![
            Command {
                prefix: Prefix::Both(":Q", ":query"),
                usage: ":Q <query>",
                description:
                    "Query the data in Structured Query Language(SQL). The table name is the file name without extension",
                function: command_query,
            },
            Command {
                prefix: Prefix::Both(":q", ":quit"),
                usage: ":q",
                description: "Quit Tabiew",
                function: command_quit,
            },
            Command {
                prefix: Prefix::Long(":goto"),
                usage: ":goto <line_index>",
                description: "Jumps to the <line_index> line",
                function: command_goto,
            },
            Command {
                prefix: Prefix::Long(":goup"),
                usage: ":goup <lines>",
                description: "Jump <lines> line(s) up",
                function: command_select_up,
            },
            Command {
                prefix: Prefix::Long(":godown"),
                usage: ":godown <lines>",
                description: "Jump <lines> line(s) down",
                function: command_select_down,
            },
            Command {
                prefix: Prefix::Long(":reset"),
                usage: ":reset",
                description: "Reset the original data frame",
                function: command_reset,
            },
            Command {
                prefix: Prefix::Long(":help"),
                usage: ":help",
                description: "Show help menu",
                function: command_help,
            },
            Command {
                prefix: Prefix::Both(":S", ":select"),
                usage: ":select <column_name(s)>",
                description: "Query current data frame for columns/functions",
                function: command_select,
            },
            Command {
                prefix: Prefix::Both(":F", ":filter"),
                usage: ":filter <condition(s)>",
                description: "Filter current data frame, keeping rows were the condition(s) match",
                function: command_filter,
            },
            Command {
                prefix: Prefix::Both(":O", ":order"),
                usage: ":order <column(s)_and_order(s)>",
                description: "Sort current data frame by column(s)",
                function: command_order,
            },
            Command {
                prefix: Prefix::Long(":tables"),
                usage: ":tables",
                description: "Show loaded data frame(s) alongside their path(s)",
                function: command_tables,
            },
        ])
    }
}

impl CommandList {
    pub fn into_exec(self) -> ExecutionTable {
        self.0
            .into_iter()
            .flat_map(|cmd| {
                match cmd.prefix {
                    Prefix::Short(short) => vec![(short, cmd.function)],
                    Prefix::Long(long) => vec![(long, cmd.function)],
                    Prefix::Both(short, long) => vec![(short, cmd.function), (long, cmd.function)],
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

pub fn command_query(
    query: &str,
    tabular: &mut Tabular,
    sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.set_data_frame(sql.execute(query)?);
    Ok(())
}
pub fn command_quit(
    _: &str,
    _: &mut Tabular,
    _: &mut SqlBackend,
    running: &mut bool,
) -> Result<(), Box<dyn Error>> {
    *running = false;
    Ok(())
}
pub fn command_goto(
    idx: &str,
    tabular: &mut Tabular,
    _: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let idx: usize = idx.parse()?;
    tabular.select(idx.saturating_sub(1));
    Ok(())
}
pub fn command_select_up(
    lines: &str,
    tabular: &mut Tabular,
    _: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.select_up(lines.parse()?);
    Ok(())
}
pub fn command_select_down(
    lines: &str,
    tabular: &mut Tabular,
    _: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.select_down(lines.parse()?);
    Ok(())
}

pub fn command_reset(
    _: &str,
    tabular: &mut Tabular,
    sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.set_data_frame(sql.default_df().ok_or("Default data frame not found")?);
    tabular.select(0);
    Ok(())
}

pub fn command_help(
    _: &str,
    tabular: &mut Tabular,
    _: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.set_data_frame(CommandList::default().into_data_frame());
    Ok(())
}

pub fn command_select(
    query: &str,
    tabular: &mut Tabular,
    _sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let mut sql = SqlBackend::new();
    sql.register("df", tabular.data_frame.clone(), PathBuf::default());
    tabular.set_data_frame(
        sql.execute(
            format!(
                "SELECT {} FROM df",
                query,
            )
            .as_str(),
        )?,
    );
    Ok(())
}

pub fn command_filter(
    query: &str,
    tabular: &mut Tabular,
    _sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let mut sql = SqlBackend::new();
    sql.register("df", tabular.data_frame.clone(), PathBuf::default());
    tabular.set_data_frame(
        sql.execute(
            format!(
                "SELECT * FROM df WHERE {}",
                query
            )
            .as_str(),
        )?,
    );
    Ok(())
}

pub fn command_order(
    query: &str,
    tabular: &mut Tabular,
    _sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let mut sql = SqlBackend::new();
    sql.register("df", tabular.data_frame.clone(), PathBuf::default());
    tabular.set_data_frame(
        sql.execute(
            format!(
                "SELECT * FROM df ORDER BY {}",
                query
            )
            .as_str(),
        )?,
    );
    Ok(())
}

pub fn command_tables(
    _query: &str,
    tabular: &mut Tabular,
    sql: &mut SqlBackend,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.set_data_frame(sql.table_df());
    Ok(())
}
