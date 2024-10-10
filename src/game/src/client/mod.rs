use crate::state::GameState;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::Value;
use std::env;

pub struct Client {
    client: aws_sdk_dynamodb::Client,
}

lazy_static::lazy_static! {
    static ref GAME_STATE_TABLE: String = format!("fomiller-dnl-{}-game-state", env::var("ENVIRONMENT").unwrap());
}

impl Client {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region("us-east-1")
            .load()
            .await;
        let client = aws_sdk_dynamodb::Client::new(&config);

        Self { client }
    }

    pub async fn try_get_game_state(self, user_id: &str) -> anyhow::Result<Option<GameState>> {
        let res = self
            .client
            .get_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .key("UserId", AttributeValue::S(user_id.to_string()))
            .send()
            .await?;

        let state: Option<GameState> = match res.item {
            Some(item) => {
                let state: GameState = serde_dynamo::from_item(item)?;
                Some(state)
            }
            None => None,
        };

        Ok(state)
    }

    pub async fn try_new_game_state(self, user_id: &str) -> anyhow::Result<Value> {
        let item = serde_dynamo::to_item(GameState {
            user_id: user_id.to_string(),
        })?;
        let res = self
            .client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(item))
            .send()
            .await?;

        let item_key: Value = serde_dynamo::from_item(
            res.item_collection_metrics()
                .unwrap()
                .item_collection_key()
                .unwrap()
                .clone(),
        )?;

        Ok(item_key)
    }
}
