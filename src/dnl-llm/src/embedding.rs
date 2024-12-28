use aws_sdk_bedrockruntime::Client as BedrockClient;
use pgvector::Vector;
use serde_json::{json, Value};

// this could probably have a builder method with a default config if values missing
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub model: String,
    pub dimension: u64,
    pub normalize: bool,
}

#[derive(Debug, Clone)]
pub struct EmbeddingEngine {
    pub client: aws_sdk_bedrockruntime::Client,
    pub config: EmbeddingConfig,
}

impl EmbeddingEngine {
    pub async fn new(config: EmbeddingConfig) -> Self {
        let aws_config = aws_config::load_from_env().await;
        let client = BedrockClient::new(&aws_config);
        Self { client, config }
    }

    pub async fn try_create_vector(&mut self, input_text: &str) -> anyhow::Result<Vector> {
        //Todo create a struct for this
        let body = json!({
            "inputText": input_text,
            "dimensions": &self.config.dimension,
            "normalize": &self.config.normalize
        });

        let res = self
            .client
            .invoke_model()
            .model_id(&self.config.model)
            .body(body.to_string().into_bytes().into())
            .accept("application/json")
            .content_type("application/json")
            .send()
            .await;

        match res {
            Ok(output) => {
                // Convert response bytes into a String
                let output_string = String::from_utf8(output.body.into_inner())
                    .expect("Response body is not valid UTF-8");

                // Parse JSON string into a serde_json::Value
                let json_response: serde_json::Value =
                    serde_json::from_str(&output_string).expect("Response body is not valid JSON");

                // Extract relevant fields from the JSON response
                let embedding = json_response.get("embedding").unwrap();
                Ok(embedding.to_vector()?)
            }
            Err(e) => {
                println!("{:?}", e.as_service_error());
                Err(anyhow::anyhow!("{:?}", e.as_service_error()))
            }
        }
    }
}

impl EmbeddingConfig {
    pub fn default() -> Self {
        Self {
            model: "amazon.titan-embed-text-v2:0".to_string(),
            dimension: 256,
            normalize: true,
        }
    }
    pub fn new(model: String, dimension: u64, normalize: bool) -> Self {
        Self {
            model,
            dimension,
            normalize,
        }
    }
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
