use super::MockData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestToolOutput { }

impl MockData for RestToolOutput {
    fn mock() -> Self { Self{} }
}

