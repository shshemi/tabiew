use std::collections::HashMap;

use polars::{
    frame::DataFrame,
    prelude::{Column, DataType, PlSmallStr},
    series::Series,
};

use crate::{
    AppResult,
    args::{Args, Type},
    misc::{polars_ext::SeriesExt, type_ext::UnwrapOrGracefulShutdown},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TypeInferer {
    int: bool,
    float: bool,
    boolean: bool,
    date: bool,
    datetime: bool,
}

impl TypeInferer {
    pub fn from_args(args: &Args) -> Self {
        if args.no_type_inference {
            Self::default()
        } else {
            let mut type_infer = TypeInferer::default();
            for t in args.infer_types.inner() {
                type_infer = match t {
                    Type::Int => type_infer.int(),
                    Type::Float => type_infer.float(),
                    Type::Boolean => type_infer.boolean(),
                    Type::Date => type_infer.date(),
                    Type::Datetime => type_infer.datetime(),
                    Type::All => type_infer.int().float().boolean().date().datetime(),
                };
            }
            type_infer
        }
    }

    pub fn update(&self, data_frame: &mut DataFrame) {
        let cast_fns = {
            let mut vec = Vec::<fn(&Series) -> AppResult<Series>>::new();
            if self.int {
                vec.push(SeriesExt::refine_to_int);
            }

            if self.float {
                vec.push(SeriesExt::refine_to_float);
            }

            if self.boolean {
                vec.push(SeriesExt::refine_to_bool);
            }

            if self.date {
                vec.push(SeriesExt::refine_to_date);
            }

            if self.datetime {
                vec.push(SeriesExt::refine_to_datetime);
            }
            vec
        };

        let updates = data_frame
            .columns()
            .iter()
            .filter(|ser| matches!(ser.dtype(), DataType::String))
            .filter_map(|col| {
                cast_fns.iter().find_map(|cast| {
                    cast(col.as_materialized_series())
                        .map(|new_ser| (col.name().to_owned(), Column::from(new_ser)))
                        .ok()
                })
            })
            .collect::<HashMap<PlSmallStr, Column>>();

        for (name, col) in updates.into_iter() {
            data_frame.replace(&name, col).unwrap_or_graceful_shutdown();
        }
    }

    pub fn int(mut self) -> Self {
        self.int = true;
        self
    }

    pub fn float(mut self) -> Self {
        self.float = true;
        self
    }

    pub fn boolean(mut self) -> Self {
        self.boolean = true;
        self
    }

    pub fn date(mut self) -> Self {
        self.date = true;
        self
    }

    pub fn datetime(mut self) -> Self {
        self.datetime = true;
        self
    }
}
