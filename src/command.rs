use std::{collections::HashMap, error::Error};

use polars::lazy::frame::LazyFrame;
use polars_sql::SQLContext;

use crate::app::Table;

pub type ExecutionFunction =
    fn(&str, &mut Table, &mut SQLContext, &mut bool) -> Result<(), Box<dyn Error>>;
pub type ExecutionTable = HashMap<&'static str, ExecutionFunction>;
pub enum Prefix {
    Short(&'static str),
    Long(&'static str),
    Both(&'static str, &'static str),
}
pub struct Command {
    pref: Prefix,
    temp: &'static str,
    desc: &'static str,
    func: ExecutionFunction,
}

pub struct CommandList(Vec<Command>);

impl Default for CommandList {
    fn default() -> Self {
        Self(vec![
            Command {
                pref: Prefix::Both(":Q", ":query"),
                temp: "<query>",
                desc: "Query the data in Structured Query Language(SQL). The table name is 'df'",
                func: command_query,
            },
            Command {
                pref: Prefix::Both(":q", ":quit"),
                temp: "",
                desc: "Quit Tabiew",
                func: command_quit,
            },
            Command {
                pref: Prefix::Long(":goto"),
                temp: "<line_index>",
                desc: "Jumps to the <line_index> line",
                func: command_select,
            },
            Command {
                pref: Prefix::Long(":moveup"),
                temp: "<lines>",
                desc: "Jump <lines> line(s) up",
                func: command_select_up,
            },
            Command {
                pref: Prefix::Long(":movedown"),
                temp: "<lines>",
                desc: "Jump <lines> line(s) down",
                func: command_select_down,
            },
        ])
    }
}

impl CommandList {
    pub fn into_exec(self) -> ExecutionTable {
        self.0
            .into_iter()
            .flat_map(|cmd| {
                match cmd.pref {
                    Prefix::Short(short) => vec![(short, cmd.func)],
                    Prefix::Long(long) => vec![(long, cmd.func)],
                    Prefix::Both(short, long) => vec![(short, cmd.func), (long, cmd.func)],
                }
                .into_iter()
            })
            .collect()
    }
}

pub fn command_query(
    query: &str,
    tabular: &mut Table,
    sql: &mut SQLContext,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.set_data_frame(sql.execute(query).and_then(LazyFrame::collect)?);
    Ok(())
}
pub fn command_quit(
    _: &str,
    _: &mut Table,
    _: &mut SQLContext,
    running: &mut bool,
) -> Result<(), Box<dyn Error>> {
    *running = false;
    Ok(())
}
pub fn command_select(
    idx: &str,
    tabular: &mut Table,
    _: &mut SQLContext,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.select(idx.parse()?);
    Ok(())
}
pub fn command_select_up(
    lines: &str,
    tabular: &mut Table,
    _: &mut SQLContext,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.select_up(lines.parse()?);
    Ok(())
}
pub fn command_select_down(
    lines: &str,
    tabular: &mut Table,
    _: &mut SQLContext,
    _: &mut bool,
) -> Result<(), Box<dyn Error>> {
    tabular.select_down(lines.parse()?);
    Ok(())
}
