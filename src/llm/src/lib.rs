use anyhow;
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{Message, SystemContentBlock},
};
use serde_json::{json, Value};

pub struct Llm {
    pub client: aws_sdk_bedrockruntime::Client,
}

impl Llm {
    pub fn new(client: aws_sdk_bedrockruntime::Client) -> Self {
        Self { client }
    }

    pub async fn converse(
        &self,
        model_id: &str,
        system: String,
        message: Message,
    ) -> anyhow::Result<ConverseOutput> {
        let response = self
            .client
            .converse()
            .model_id(model_id)
            .system(SystemContentBlock::Text(system))
            .messages(message)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn try_create_embed(
        &self,
        input_text: &str,
        dimensions: i64,
        normalize: bool,
    ) -> anyhow::Result<Vec<f32>> {
        let body = json!({
            "inputText": input_text,
            "dimensions": dimensions,
            "normalize": normalize
        });

        let res = self
            .client
            .invoke_model()
            .model_id("amazon.titan-embed-text-v2:0")
            .body(body.to_string().into_bytes().into())
            .accept("application/json")
            .content_type("application/json")
            .send()
            .await;

        let embedding_prompt = match res {
            Ok(output) => {
                // Convert response bytes into a String
                let output_string = String::from_utf8(output.body.into_inner())
                    .expect("Response body is not valid UTF-8");

                // Parse JSON string into a serde_json::Value
                let json_response: serde_json::Value =
                    serde_json::from_str(&output_string).expect("Response body is not valid JSON");

                // Extract relevant fields from the JSON response
                let embedding = json_response.get("embedding").unwrap();
                let x = Self::value_to_f32_slice(embedding);
                Ok(x)
            }
            Err(e) => {
                println!("{:?}", e.as_service_error());
                Err(anyhow::anyhow!("{:?}", e.as_service_error()))
            }
        };

        embedding_prompt.unwrap()
    }

    #[allow(dead_code)]
    fn value_to_f32_slice(value: &Value) -> anyhow::Result<Vec<f32>> {
        if let Value::Array(array) = value {
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

    pub fn get_converse_output_text(output: ConverseOutput) -> anyhow::Result<String> {
        let text = output
            .output()
            .ok_or_else(|| anyhow::anyhow!("no output"))?
            .as_message()
            .map_err(|_| anyhow::anyhow!("output not a message"))?
            .content()
            .first()
            .ok_or_else(|| anyhow::anyhow!("no content in message"))?
            .as_text()
            .map_err(|_| anyhow::anyhow!("content is not text"))?
            .to_string();
        Ok(text)
    }

    pub fn create_input(contexts: Vec<String>, instructions: String, prompt: String) -> String {
        let mut input_context = String::new();
        for context in contexts {
            input_context.push_str(&context)
        }
        let input = format!(
            "{}\n<Context>{}</Context>\n<Prompt>\n{}\n</Prompt>",
            instructions, input_context, prompt
        );

        input
    }
}
