pub mod message;
pub mod schema;
pub mod state_component;
pub mod user;
pub mod weapon;
pub mod encounter;

use crate::state_component::StateComponent;
use crate::encounter::EncounterQuery;
use anyhow::anyhow;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{operation::put_item::PutItemOutput, types::PutRequest};
use aws_sdk_dynamodb::{
    operation::{batch_write_item::BatchWriteItemInput, get_item::GetItemOutput},
    types::WriteRequest,
};
use dnl_sort_keys::game::GameState;
use dnl_sort_keys::prelude::*;
use lambda_http::tracing::{debug, info};
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

pub struct Store {
    client: aws_sdk_dynamodb::Client,
}

lazy_static::lazy_static! {
    static ref GAME_STATE_TABLE: String = format!("fomiller-dnl-{}-game-state", env::var("ENVIRONMENT").unwrap());
}
const SQID_ALPHABET: &str = "k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt";

impl Store {
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
    pub async fn try_generic_begins_with_query(
        &self,
        primary_key: String,
        sk_prefix: String,
    ) -> anyhow::Result<QueryOutput> {
        let res = self
            .client
            .query()
            .table_name(GAME_STATE_TABLE.to_string())
            .key_condition_expression("#pk = :user_id AND begins_with(#sk, :sk_prefix)")
            .expression_attribute_names("#pk", "UserId")
            .expression_attribute_names("#sk", "StateComponent")
            .expression_attribute_names("#state", "State")
            .expression_attribute_values(":user_id", AttributeValue::S(primary_key))
            .expression_attribute_values(":sk_prefix", AttributeValue::S(sk_prefix))
            .projection_expression("#state")
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
            state_component: RootSortKeyBuilder::new()
                .id(user_id)
                .message(MessageSortKey::LastMessageToken)
                .build(),
            state: Some(token),
            ..Default::default()
        })?;

        self.try_generic_put(last_message_token).await?;

        Ok(())
    }

    pub async fn try_save_active_game_id(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> anyhow::Result<()> {
        let sk = SortKeyFactory::new(user_id)
            .create_user_active_game_sk()
            .build();

        let active_game_id = serde_dynamo::to_item(StateComponent {
            user_id: user_id.to_string(),
            state_component: sk,
            state: Some(game_id),
            ..Default::default()
        })?;

        self.try_generic_put(active_game_id).await?;

        Ok(())
    }

    pub async fn try_save_encounter(
        &self,
        user_id: &str,
        game_id: &str,
        encounter: EncounterSortKey,
        level: u8,
        round: u8,
        state: Value,
    ) -> anyhow::Result<()> {
        let sk = SortKeyFactory::new(user_id)
            .create_encounter_sk(game_id, round, level, encounter)
            .build();

        let encounter = serde_dynamo::to_item(StateComponent {
            user_id: user_id.to_string(),
            state_component: sk,
            state: Some(state),
            ..Default::default()
        })?;

        self.try_generic_put(encounter).await?;

        Ok(())
    }

    pub async fn try_get_last_message_token(&self, user_id: &str) -> anyhow::Result<QueryOutput> {
        let sort_key = RootSortKeyBuilder::new()
            .id(user_id)
            .message(MessageSortKey::LastMessageToken)
            .build();

        let res = self
            .try_generic_query(user_id.to_string(), sort_key)
            .await?;

        Ok(res)
    }

    pub async fn try_get_encounters(&self, user_id: &str, game_id: &str, level: &str, encounter: &str) -> anyhow::Result<Vec<EncounterQuery>> {
        let mut chars = encounter.chars();
        let encounter = match chars.next() {
            Some(first_char) => first_char.to_uppercase().chain(chars).collect(),
            None => String::new(), // Return empty string if input is empty
        };
        let sk = format!("{}#Game#Level#{}#Encounter#{}#Round#",game_id, level, encounter);
        let res = self
            .try_generic_begins_with_query(user_id.to_string(), sk)
            .await?;
        
        let items = res.items.unwrap();
        debug!("Items: {:?}", items);
        
        let ctxs: Vec<EncounterQuery> = serde_dynamo::from_items(items)?;
        Ok(ctxs)
    }
    
    pub async fn try_get_active_game_id(&self, user_id: &str) -> anyhow::Result<String> {
        let sk = SortKeyFactory::new(&user_id)
            .create_user_active_game_sk()
            .build();

        let res = self
            .try_generic_query(user_id.to_string(), sk.clone())
            .await?;

        let items = res.items.expect(format!("Could not find {}", sk).as_str());

        debug!("QUERY: {:?}", items);

        let game_id = items
            .first()
            .unwrap()
            .get_key_value("State")
            .expect("State for ActiveGameId not found")
            .1
            .as_s()
            .unwrap()
            .to_owned();

        Ok(game_id)
    }

    pub async fn try_new_game(&self, user_id: &str) -> anyhow::Result<()> {
        let game_id = try_create_sqid(None)?;

        let factory = SortKeyFactory::new(user_id);

        let mut sort_keys: Vec<RootSortKeyBuilder> = Vec::new();

        sort_keys.extend(factory.create_all_entity_inventory_sks(&game_id));
        sort_keys.extend(factory.create_all_entity_actions_sks(&game_id));
        sort_keys.extend(factory.create_all_entity_stats_sks(&game_id));

        self.try_generic_batch_write_root_sks(user_id, sort_keys)
            .await?;

        self.try_save_active_game_id(user_id, &game_id).await?;

        Ok(())
    }

    pub async fn try_generic_batch_write_root_sks(
        &self,
        user_id: &str,
        sort_keys: Vec<RootSortKeyBuilder>,
    ) -> anyhow::Result<()> {
        info!("Batch Write Item Count: {}", sort_keys.len());

        let mut items: Vec<HashMap<String, AttributeValue>> = Vec::new();

        let components: Vec<StateComponent<HashMap<String, Value>>> = sort_keys
            .iter()
            .map(|sk| StateComponent {
                user_id: user_id.to_string(),
                state_component: sk.build(),
                state: None,
                ..Default::default()
            })
            .collect();

        for i in components {
            match serde_dynamo::to_item(i) {
                Ok(item) => items.push(item),
                Err(e) => return Err(anyhow!(e)),
            }
        }

        let mut write_requests = Vec::new();
        while !items.is_empty() {
            debug!("Items: {:?}", items);
            debug!("Items Count: {:?}", items.len());

            // if there are previous unprocessed_items then write_requests will
            // not be empty so we will subtract that maximum value from the maximum batch value
            let batch: Vec<_> = items
                .drain(..(25 - write_requests.len()).min(items.len()))
                .collect();

            for item in batch {
                let put_request = PutRequest::builder().set_item(Some(item)).build()?;
                let write_request = WriteRequest::builder().put_request(put_request).build();
                write_requests.push(write_request);
            }

            // Prepare the batch write input
            let request = BatchWriteItemInput::builder()
                .request_items(GAME_STATE_TABLE.to_string(), write_requests.clone())
                .build()?;

            match self
                .client
                .batch_write_item()
                .set_request_items(request.request_items)
                .send()
                .await
            {
                Ok(request) => {
                    debug!("Batch Write Request: {:?}", request);
                    if let Some(unprocessed) = request.unprocessed_items {
                        if !unprocessed.is_empty() {
                            // if there are unprocessed_items
                            info!("Unprocessed Batch Items: {:?}", unprocessed);

                            let key = GAME_STATE_TABLE.as_str();

                            let requests = unprocessed
                                .get(key)
                                .expect(&format!("{} key not found in unprocessed requests.", key))
                                .to_owned();

                            write_requests = requests
                        } else {
                            // if there are not unprocessed_items
                            write_requests.clear();
                            info!("Batch write successful!");
                        }
                    }
                }
                Err(e) => eprintln!("Error during batch write: {:?}", e),
            }
        }
        Ok(())
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
