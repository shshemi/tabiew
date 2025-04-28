use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use calamine::{Data, Range, Reader, open_workbook_auto_from_rs};
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, NamedFrom, PlSmallStr},
    series::Series,
};

use crate::{
    AppResult,
    misc::{globals::stdin, polars_ext::SafeInferSchema, type_ext::SnakeCaseNameGenExt},
};

use super::{InputSource, NamedFrames, ReadToDataFrames};

pub struct ExcelToDataFarmes;

impl ReadToDataFrames for ExcelToDataFarmes {
    fn named_frames(&self, input: InputSource) -> AppResult<NamedFrames> {
        let buffer = match input {
            InputSource::File(path) => Cursor::new(std::fs::read(path)?),
            InputSource::Stdin => {
                let mut buf = Vec::new();
                stdin().read_to_end(&mut buf).unwrap();
                Cursor::new(buf)
            }
        };
        Ok(open_workbook_auto_from_rs(buffer)?
            .worksheets()
            .into_iter()
            .map(|(name, sheet)| {
                let mut df = sheet_to_data_frame(sheet);
                df.safe_infer_schema();
                (name, df)
            })
            .collect_vec()
            .into_boxed_slice())
    }
}

fn sheet_to_data_frame(sheet: Range<Data>) -> DataFrame {
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
    DataFrame::from_iter(
        columns
            .into_iter()
            .enumerate()
            .map(|(idx, column)| Series::new(col_letter(col_offset + idx).into(), column)),
    )
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
