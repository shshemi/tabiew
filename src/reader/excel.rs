use std::io::{Cursor, Read};

use calamine::{Data, Range, Reader, open_workbook_auto_from_rs};
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, NamedFrom},
    series::Series,
};

use crate::{
    AppResult,
    args::{Args, InferSchema},
    misc::{
        globals::stdin,
        polars_ext::{FullInferSchema, SafeInferSchema},
    },
};

use super::{NamedFrames, ReadToDataFrames, Source};

pub struct ExcelToDataFarmes {
    infer_schema: InferSchema,
}

impl ExcelToDataFarmes {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
        }
    }
}

impl ReadToDataFrames for ExcelToDataFarmes {
    fn named_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let buffer = match input {
            Source::File(path) => Cursor::new(std::fs::read(path)?),
            Source::Stdin => {
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
                match self.infer_schema {
                    InferSchema::Fast | InferSchema::Safe => {
                        df.safe_infer_schema();
                    }
                    InferSchema::Full => {
                        df.full_infer_schema();
                    }
                    _ => (),
                }
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
