#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub name: String,
    pub installed: bool,
    pub loaded: bool,
    pub description: String,
}
