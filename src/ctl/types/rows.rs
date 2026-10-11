use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rows {
    pub offset: usize,
    pub total: usize,
    pub records: String,
}

impl Rows {
    pub fn to_json(&self) -> AppResult<Value> {
        Ok(json!({
            "offset": self.offset,
            "total": self.total,
            "rows": serde_json::from_str::<Value>(&self.records)?,
        }))
    }
}
