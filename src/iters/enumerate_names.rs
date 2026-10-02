pub fn enumerate_names(base: &str) -> impl Iterator<Item = String> {
    let mut stage = 0;
    std::iter::from_fn(move || {
        stage += 1;
        match stage {
            1 => base.to_owned().into(),
            2.. => format!("{}_{}", base, stage).into(),
            _ => unreachable!(),
        }
    })
}
