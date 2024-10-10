use crate::state::*;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::tracing::info;
use serde_json::json;
use std::collections::HashMap;
use std::convert::From;
use std::env;
use uuid::Uuid;

pub struct Client {
    client: aws_sdk_dynamodb::Client,
}

lazy_static::lazy_static! {
    static ref GAME_STATE_TABLE: String = format!("fomiller-dnl-{}-game-state", env::var("ENVIRONMENT").unwrap());
}
const SQID_ALPHABET: &str = "k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt";

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
        let sqids = sqids::Sqids::builder()
            .min_length(10)
            .alphabet(SQID_ALPHABET.chars().collect())
            .build()?;

        let new_game_id = sqids.encode(&[1, 2, 3])?;

        let user = serde_dynamo::to_item(User {
            user_id: user_id.to_string(),
            state_component: format!("{}#ActiveGameId", user_id.to_string()),
            active_game_id: Some(new_game_id.to_string()),
            games: Some(vec![new_game_id.to_string()]),
        })?;

        let state_comp_wep = serde_dynamo::to_item(StateComponent {
            user_id: user_id.to_string(),
            state_component: format!("{}#Item#Weapon", new_game_id.to_string()),
            state: Some(vec![StateComponentWeapon {
                name: "great-sword".to_string(),
                price: 100,
                damage: 69,
            }]),
        })?;

        // let weapon = HashMap::from([
        //     (
        //         "UserId".to_string(),
        //         AttributeValue::S(String::from(user_id)),
        //     ),
        //     (
        //         "StateComponent".to_string(),
        //         AttributeValue::S(format!("{}#Items#Weapons", new_game_id.to_string())),
        //     ),
        //     (
        //         "State".to_string(),
        //         serde_dynamo::to_item(
        //             StateComponentWeapon {}
        //                 & Weapon {
        //                     name: "sword".to_string(),
        //                     damage: 10,
        //                     price: 5,
        //                 },
        //         )?,
        //     ),
        // ]);

        self.client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(user))
            .send()
            .await?;

        self.client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(state_comp_wep))
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
