use crate::embedding::ToVector;
use anyhow;
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{Message, SystemContentBlock},
};

use pgvector::Vector;
use serde_json::json;

pub struct LlmHandler {
    pub client: aws_sdk_bedrockruntime::Client,
}

impl LlmHandler {
    pub fn new(client: aws_sdk_bedrockruntime::Client) -> Self {
        Self { client }
    }

    pub async fn converse(
        &self,
        model_id: &str,
        system: String,
        messages: Vec<Message>,
    ) -> anyhow::Result<ConverseOutput> {
        let response = self
            .client
            .converse()
            .model_id(model_id)
            .system(SystemContentBlock::Text(system))
            .set_messages(Some(messages))
            .send()
            .await?;

        Ok(response)
    }

    pub async fn try_create_vector(
        &self,
        input_text: &str,
        dimensions: i64,
        normalize: bool,
    ) -> anyhow::Result<Vector> {
        //Todo create a struct for this
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
