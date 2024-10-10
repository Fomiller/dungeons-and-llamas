use crate::state::{GameState, User, Weapon};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::tracing::info;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

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

    pub async fn try_get_game_state(&self, user_id: &str) -> anyhow::Result<Option<GameState>> {
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

    pub async fn try_new_game_state(&self, user_id: &str) -> anyhow::Result<()> {
        let new_game_id = Uuid::new_v4();
        let user = serde_dynamo::to_item(User {
            user_id: user_id.to_string(),
            active_game_id: new_game_id.to_string(),
        })?;

        let weapon = HashMap::from([
            (
                "UserId".to_string(),
                AttributeValue::S(String::from(user_id)),
            ),
            (
                "StateComponent".to_string(),
                AttributeValue::S(format!("{}#Items#Weapons", new_game_id.to_string())),
            ),
            (
                "State".to_string(),
                AttributeValue::S(serde_json::to_string(&Weapon {
                    name: "sword".to_string(),
                    damage: 10,
                    price: 5,
                })?),
            ),
        ]);

        self.client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(user))
            .send()
            .await?;

        self.client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(weapon))
            .send()
            .await?;

        Ok(())
    }

    pub async fn try_find_user(&self, user_id: &str) -> anyhow::Result<Option<GameState>> {
        let res = self
            .client
            .get_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .key("UserId", AttributeValue::S(user_id.to_string()))
            .send()
            .await?;

        match res.item {
            Some(item) => {
                let state: GameState = serde_dynamo::from_item(item)?;
                Ok(Some(state))
            }
            None => {
                info!("New user created: {}", user_id);
                Ok(None)
            }
        }
    }

    pub async fn try_create_user(&self, user_id: &str) -> anyhow::Result<()> {
        let item = serde_dynamo::to_item(GameState {
            user_id: user_id.to_string(),
        })?;

        self.client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }
}
