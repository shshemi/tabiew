#[inline]
pub fn get_num() -> usize {
    std::thread::available_parallelism()
        .map(Into::into)
        .unwrap_or(1)
}
