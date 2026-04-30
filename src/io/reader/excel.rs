use std::io::Cursor;

use calamine::{Data, Range, Reader, open_workbook_auto_from_rs};
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, Column},
};

use crate::{AppResult, args::Args, io::reader::ReaderSource, misc::stdin::stdin};

use super::{NamedFrames, ReadToDataFrames};

#[derive(Debug, Default)]
pub struct ExcelToDataFrames;

impl ExcelToDataFrames {
    pub fn from_args(_args: &Args) -> Self {
        Self
    }
}

impl ReadToDataFrames for ExcelToDataFrames {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        Ok(match input {
            ReaderSource::File(path) => {
                open_workbook_auto_from_rs(Cursor::new(std::fs::read(path)?))?
                    .worksheets()
                    .into_iter()
                    .map(|(name, sheet)| {
                        let df = sheet_to_data_frame(sheet)?;
                        Ok((name, df))
                    })
                    .collect::<AppResult<Vec<_>>>()?
                    .into_boxed_slice()
            }
            ReaderSource::Stdin => open_workbook_auto_from_rs(stdin())?
                .worksheets()
                .into_iter()
                .map(|(name, sheet)| {
                    let df = sheet_to_data_frame(sheet)?;
                    Ok((name, df))
                })
                .collect::<AppResult<Vec<_>>>()?
                .into_boxed_slice(),
        })
    }
}

fn sheet_to_data_frame(sheet: Range<Data>) -> AppResult<DataFrame> {
    let col_offset = sheet.start().unwrap_or_default().1 as usize;
    let mut columns = vec![Vec::new(); sheet.width()];
    for row in sheet.rows() {
        for (idx, cell) in row.iter().enumerate() {
            columns[idx].push(match cell {
                Data::Empty => AnyValue::Null,
                _ => AnyValue::StringOwned(cell.to_string().into()),
            });
        }
    }
    Ok(DataFrame::new_infer_height(
        columns
            .into_iter()
            .enumerate()
            .map(|(idx, column)| Column::new(col_letter(col_offset + idx).into(), column))
            .collect_vec(),
    )?)
}

fn col_letter(mut col_index: usize) -> String {
    let mut col_letter = String::new();
    loop {
        let rem = (col_index % 26) as u8;
        col_letter.insert(0, (b'A' + rem) as char);
        col_index /= 26;
        if col_index == 0 {
            break;
        }
        col_index -= 1;
    }
    col_letter
}
