use pgvector::Vector;
use serde_json::Value;

// this could probably have a builder method with a default config if values missing
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub model: String,
    pub dimension: u64,
    pub normalize: bool,
}

pub trait ToF32Slice {
    fn value_to_f32_slice(&self) -> anyhow::Result<Vec<f32>>;
}

pub trait ToVector {
    fn to_vector(&self) -> anyhow::Result<Vector>;
}

impl ToF32Slice for serde_json::Value {
    fn value_to_f32_slice(&self) -> anyhow::Result<Vec<f32>> {
        if let Value::Array(array) = self {
            // Try to parse each element as f32
            let result: Result<Vec<f32>, _> = array
                .iter()
                .map(|v| {
                    v.as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Value is not a valid number"))
                })
                .map(|num| num.and_then(|n| Ok(n as f32)))
                .collect();

            result.map_err(|e| anyhow::anyhow!("Error parsing array: {}", e))
        } else {
            Err(anyhow::anyhow!("Value is not an array"))
        }
    }
}

impl ToVector for serde_json::Value {
    fn to_vector(&self) -> anyhow::Result<Vector> {
        Ok(Vector::from(self.value_to_f32_slice()?))
    }
}
