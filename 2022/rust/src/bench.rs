use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bench {
    pub day: usize,
    pub part: usize,
    pub time: usize,
}
