pub struct TableNameGenerator<'a> {
    base: &'a str,
    stage: u32,
}

impl<'a> TableNameGenerator<'a> {
    pub fn with(base: &'a str) -> Self {
        Self { base, stage: 0 }
    }
}

impl Iterator for TableNameGenerator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stage += 1;
        match self.stage {
            1 => self.base.to_owned().into(),
            2.. => format!("{}_{}", self.base, self.stage).into(),
            _ => unimplemented!(),
        }
    }
}

pub trait TableNameGeneratorExt {
    fn snake_case_names(&self) -> TableNameGenerator<'_>;
}

impl TableNameGeneratorExt for str {
    fn snake_case_names(&self) -> TableNameGenerator<'_> {
        TableNameGenerator::with(self)
    }
}
