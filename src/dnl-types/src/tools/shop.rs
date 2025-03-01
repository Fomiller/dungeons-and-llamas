use super::MockData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolOutput { }

impl MockData for ShopToolOutput {
    fn mock() -> Self { Self{} }
}

