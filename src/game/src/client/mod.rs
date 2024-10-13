use crate::state::*;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::tracing::info;
use rand::Rng;
use std::collections::HashMap;
use std::env;

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

    pub async fn try_generic_query(
        &self,
        primary_key: String,
        sort_key: String,
    ) -> anyhow::Result<QueryOutput> {
        let res = self
            .client
            .query()
            .table_name(GAME_STATE_TABLE.to_string())
            .key_condition_expression("#pk = :user_id AND #sk = :sort_key")
            .expression_attribute_names("#pk", "UserId")
            .expression_attribute_names("#sk", "StateComponent")
            .expression_attribute_values(":user_id", AttributeValue::S(primary_key))
            .expression_attribute_values(":sort_key", AttributeValue::S(sort_key))
            .send()
            .await?;
        Ok(res)
    }

    pub async fn try_generic_get(&self, key: String) -> anyhow::Result<GetItemOutput> {
        let res = self
            .client
            .get_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .key("UserId", AttributeValue::S(key))
            .send()
            .await?;
        Ok(res)
    }

    pub async fn try_generic_put(
        &self,
        item: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<PutItemOutput> {
        let res = self
            .client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(item))
            .send()
            .await?;
        Ok(res)
    }

    pub async fn try_get_game_state(&self, user_id: &str) -> anyhow::Result<Option<GameState>> {
        let res = self.try_generic_get(user_id.to_string()).await?;

        let state: Option<GameState> = match res.item {
            Some(item) => {
                let state: GameState = serde_dynamo::from_item(item)?;
                Some(state)
            }
            None => None,
        };

        Ok(state)
    }

    pub async fn try_new_game_state(&self, user_id: &str, name: &str) -> anyhow::Result<()> {
        let new_game_id = try_create_sqid(None)?;

        let user = serde_dynamo::to_item(User {
            user_id: user_id.to_string(),
            name: name.to_string(),
            state_component: UserSortKeys::ActiveGameId(user_id).to_string(),
            active_game_id: Some(new_game_id.to_string()),
            games: Some(vec![new_game_id.to_string()]),
        })?;

        let state_comp_wep = serde_dynamo::to_item(StateComponent {
            user_id: user_id.to_string(),
            state_component: ItemSortKeys::Weapons(&new_game_id).to_string(),
            state: Some(vec![StateComponentWeapon {
                name: "great-sword".to_string(),
                price: 100,
                damage: 69,
            }]),
        })?;

        self.try_generic_put(user).await?;
        self.try_generic_put(state_comp_wep).await?;

        Ok(())
    }

    pub async fn try_find_user(&self, user_id: &str) -> anyhow::Result<Option<GameState>> {
        let res = self.try_generic_get(user_id.to_string()).await?;

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

        self.try_generic_put(item).await?;

        Ok(())
    }

    pub async fn try_save_message_token(&self, user_id: &str, token: &str) -> anyhow::Result<()> {
        let last_message_token = serde_dynamo::to_item(StateComponent {
            user_id: user_id.to_string(),
            state_component: MessageSortKeys::LastMessageToken(user_id).to_string(),
            state: Some(token),
        })?;

        self.try_generic_put(last_message_token).await?;

        Ok(())
    }

    pub async fn try_get_last_message_token(&self, user_id: &str) -> anyhow::Result<QueryOutput> {
        let sort_key = MessageSortKeys::LastMessageToken(user_id).to_string();

        let res = self
            .try_generic_query(user_id.to_string(), sort_key)
            .await?;

        Ok(res)
    }
}
pub fn try_create_sqid(min_length: Option<u8>) -> anyhow::Result<String> {
    let sqids = sqids::Sqids::builder()
        .min_length(min_length.unwrap_or(10))
        .alphabet(SQID_ALPHABET.chars().collect())
        .build()?;

    Ok(sqids.encode(&[
        rand::thread_rng().gen_range(0..1000),
        rand::thread_rng().gen_range(0..1000),
        rand::thread_rng().gen_range(0..1000),
        rand::thread_rng().gen_range(0..1000),
    ])?)
}
