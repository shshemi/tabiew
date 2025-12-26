pub struct SnakeCaseNameGen<'a> {
    base: &'a str,
    stage: u32,
}

impl<'a> SnakeCaseNameGen<'a> {
    pub fn with(base: &'a str) -> Self {
        Self { base, stage: 0 }
    }
}

impl Iterator for SnakeCaseNameGen<'_> {
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

pub trait SnakeCaseNameGenExt {
    fn snake_case_names(&self) -> SnakeCaseNameGen<'_>;
}

impl SnakeCaseNameGenExt for str {
    fn snake_case_names(&self) -> SnakeCaseNameGen<'_> {
        SnakeCaseNameGen::with(self)
    }
}
