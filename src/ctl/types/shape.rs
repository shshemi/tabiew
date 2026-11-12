use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    pub height: usize,
    pub width: usize,
}
