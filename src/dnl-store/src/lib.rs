pub mod encounter;
pub mod message;
pub mod schema;
pub mod state_component;
pub mod user;
pub mod weapon;

use crate::encounter::EncounterQuery;
use crate::state_component::StateComponent;

use dnl_sort_keys::prelude::*;
use dnl_types::api::request::NewGameData;
use dnl_types::api::response::NewGameResponse;
use dnl_types::entity::stats::get_base_stats_by_name;
use dnl_types::scenarios::battle::BattleScenarioEnemy;
use dnl_types::settings::Settings;

use std::collections::HashMap;
use std::env;

use anyhow::{anyhow, Context};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::operation::update_item::UpdateItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{operation::put_item::PutItemOutput, types::PutRequest};
use aws_sdk_dynamodb::{
    operation::{batch_write_item::BatchWriteItemInput, get_item::GetItemOutput},
    types::TransactWriteItem,
    types::Update,
    types::WriteRequest,
};
use lambda_http::tracing::{debug, info};
use rand::Rng;
use strum::IntoEnumIterator;

pub type Item = HashMap<String, AttributeValue>;

#[derive(Debug, Clone)]
pub struct UpdateItem {
    pub sort_key: RootSortKeyBuilder,
    pub item: (String, String),
}

#[derive(Debug, Clone)]
pub struct Store {
    client: aws_sdk_dynamodb::Client,
    user_id: String,
}

lazy_static::lazy_static! {
    static ref GAME_STATE_TABLE: String = format!("fomiller-dnl-{}-game-state", env::var("ENVIRONMENT").unwrap());
}
const SQID_ALPHABET: &str = "k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt";

impl Store {
    pub async fn new(user_id: &str) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region("us-east-1")
            .load()
            .await;
        let client = aws_sdk_dynamodb::Client::new(&config);

        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn try_generic_update(
        &self,
        primary_key: &str,
        sort_key: &str,
        updates: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<UpdateItemOutput> {
        let mut expression_attribute_names = HashMap::new();
        let mut expression_attribute_values = HashMap::new();
        let mut update_expressions = Vec::new();

        for (field, value) in &updates {
            let placeholder_name = format!("#{}", field);
            let placeholder_value = format!(":{}", field);

            expression_attribute_names.insert(placeholder_name.clone(), field.clone());
            expression_attribute_values.insert(placeholder_value.clone(), value.clone());

            update_expressions.push(format!("{} = {}", placeholder_name, placeholder_value));
        }

        let update_expression = format!("SET {}", update_expressions.join(", "));

        let res = self
            .client
            .update_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .key("UserId", AttributeValue::S(primary_key.to_string()))
            .key("StateComponent", AttributeValue::S(sort_key.to_string()))
            .update_expression(update_expression)
            .set_expression_attribute_names(Some(expression_attribute_names))
            .set_expression_attribute_values(Some(expression_attribute_values))
            .send()
            .await
            .context("aws dynamodb update item failed")?;

        Ok(res)
    }

    pub async fn try_generic_query(
        &self,
        sort_key: String,
        attributes: &Vec<&str>,
    ) -> anyhow::Result<QueryOutput> {
        let mut expression_attribute_names = HashMap::new();

        expression_attribute_names.insert("#pk".to_string(), "UserId".to_string());
        expression_attribute_names.insert("#sk".to_string(), "StateComponent".to_string());

        let mut aliased_attributes = Vec::new();
        for (i, attr) in attributes.iter().enumerate() {
            let alias = format!("#attr{}", i);
            expression_attribute_names.insert(alias.to_string(), attr.to_string());
            aliased_attributes.push(alias);
        }

        let projection_expression = if aliased_attributes.is_empty() {
            None
        } else {
            Some(aliased_attributes.join(", "))
        };

        let res = self
            .client
            .query()
            .table_name(GAME_STATE_TABLE.to_string())
            .key_condition_expression("#pk = :user_id AND #sk = :sort_key")
            .set_expression_attribute_names(Some(expression_attribute_names))
            .expression_attribute_values(":user_id", AttributeValue::S(self.user_id.to_string()))
            .expression_attribute_values(":sort_key", AttributeValue::S(sort_key))
            .set_projection_expression(projection_expression)
            .send()
            .await
            .context("aws dynamodb client query failed")?;

        Ok(res)
    }
    pub async fn try_generic_begins_with_query(
        &self,
        primary_key: String,
        sk_prefix: String,
        attributes: Vec<&str>,
    ) -> anyhow::Result<QueryOutput> {
        let mut expression_attribute_names = HashMap::new();

        expression_attribute_names.insert("#pk".to_string(), "UserId".to_string());
        expression_attribute_names.insert("#sk".to_string(), "StateComponent".to_string());

        let mut aliased_attributes = Vec::new();
        for (i, attr) in attributes.iter().enumerate() {
            let alias = format!("#attr{}", i);
            expression_attribute_names.insert(alias.to_string(), attr.to_string());
            aliased_attributes.push(alias);
        }

        let projection_expression = if aliased_attributes.is_empty() {
            None
        } else {
            Some(aliased_attributes.join(", "))
        };

        let res = self
            .client
            .query()
            .table_name(GAME_STATE_TABLE.to_string())
            .key_condition_expression("#pk = :user_id AND begins_with(#sk, :sk_prefix)")
            .set_expression_attribute_names(Some(expression_attribute_names))
            .expression_attribute_values(":user_id", AttributeValue::S(primary_key))
            .expression_attribute_values(":sk_prefix", AttributeValue::S(sk_prefix))
            .set_projection_expression(projection_expression)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn try_generic_get(&self, sort_key: &str) -> anyhow::Result<GetItemOutput> {
        let res = self
            .client
            .get_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .key("UserId", AttributeValue::S(self.user_id.clone()))
            .key("StateComponent", AttributeValue::S(sort_key.to_string()))
            .send()
            .await?;
        Ok(res)
    }

    pub async fn try_generic_put(
        &self,
        item: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<PutItemOutput> {
        info!("GEN-PUT: {:?}", item);
        let res = self
            .client
            .put_item()
            .table_name(GAME_STATE_TABLE.to_string())
            .set_item(Some(item))
            .send()
            .await?;
        Ok(res)
    }

    // pub async fn try_get_game_state(&self) -> anyhow::Result<Option<GameState>> {
    //     let res = self.try_generic_get().await?;
    //
    //     let state: Option<GameState> = match res.item {
    //         Some(item) => {
    //             let state: GameState = serde_dynamo::from_item(item)?;
    //             Some(state)
    //         }
    //         None => None,
    //     };
    //
    //     Ok(state)
    // }

    // pub async fn try_find_user(&self) -> anyhow::Result<Option<GameState>> {
    //     let res = self.try_generic_get().await?;
    //
    //     match res.item {
    //         Some(item) => {
    //             let state: GameState = serde_dynamo::from_item(item)?;
    //             Ok(Some(state))
    //         }
    //         None => {
    //             info!("New user created: {}", self.user_id);
    //             Ok(None)
    //         }
    //     }
    // }

    pub async fn try_create_user(&self) -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("user_id", &self.user_id);
        let item = serde_dynamo::to_item(map)?;

        self.try_generic_put(item).await?;

        Ok(())
    }

    pub async fn try_save_message_token(&self, token: &str) -> anyhow::Result<()> {
        let mut state_component: Item = serde_dynamo::to_item(StateComponent {
            user_id: self.user_id.to_string(),
            state_component: RootSortKeyBuilder::new()
                .id(&self.user_id)
                .message(MessageSortKey::LastMessageToken)
                .build(),
            ..Default::default()
        })?;

        let mut map = HashMap::new();
        map.insert("last_message_token", token);
        let item: Item = serde_dynamo::to_item(map)?;

        state_component.extend(item);

        self.try_generic_put(state_component).await?;

        Ok(())
    }

    pub async fn try_save_game_settings(
        &self,
        game_id: &str,
        settings: Settings,
    ) -> anyhow::Result<()> {
        let sk = SortKeyFactory::new(&self.user_id)
            .create_game_settings_sk(game_id)
            .build();

        let mut state_component: Item = serde_dynamo::to_item(StateComponent {
            user_id: self.user_id.to_string(),
            state_component: sk,
            ..Default::default()
        })?;

        let item: Item = serde_dynamo::to_item(settings)?;

        state_component.extend(item);

        self.try_generic_put(state_component).await?;

        Ok(())
    }

    pub async fn try_save_new_game_state(&self) -> anyhow::Result<()> {
        let game_id = self.try_get_active_game_id().await?;
        let sk = RootSortKeyBuilder::create_state_sk(&game_id).build();

        let mut state_component: Item = serde_dynamo::to_item(StateComponent {
            user_id: self.user_id.to_string(),
            state_component: sk,
            ..Default::default()
        })?;

        let mut map = HashMap::new();

        map.insert("round".to_string(), 1.to_string());
        map.insert("level".to_string(), 1.to_string());
        map.insert(
            "curr_encounter".to_string(),
            EncounterSortKey::NewGame.to_string(),
        );

        let item: Item = serde_dynamo::to_item(map)?;

        state_component.extend(item);

        self.try_generic_put(state_component).await?;

        Ok(())
    }

    pub async fn try_save_active_game_id(&self, game_id: &str) -> anyhow::Result<()> {
        let sk = SortKeyFactory::new(&self.user_id)
            .create_user_active_game_sk()
            .build();

        let mut state_component: Item = serde_dynamo::to_item(StateComponent {
            user_id: self.user_id.to_string(),
            state_component: sk,
            ..Default::default()
        })?;

        // let _game_id = AttributeValue::S(game_id.to_string());
        let mut map = HashMap::new();
        map.insert("game_id".to_string(), game_id);
        let item: Item = serde_dynamo::to_item(map)?;

        state_component.extend(item);

        self.try_generic_put(state_component).await?;

        Ok(())
    }

    pub async fn try_save_encounter(
        &self,
        game_id: &str,
        encounter: EncounterSortKey,
        level: u8,
        round: u8,
        state: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<()> {
        let sk = SortKeyFactory::new(&self.user_id)
            .create_encounter_sk(game_id, round, level, encounter)
            .build();

        let mut state_component: Item = serde_dynamo::to_item(StateComponent {
            user_id: self.user_id.to_string(),
            state_component: sk,
            ..Default::default()
        })?;

        state_component.extend(state);

        self.try_generic_put(state_component).await?;

        Ok(())
    }

    pub async fn try_get_last_message_token(&self) -> anyhow::Result<QueryOutput> {
        let sort_key = RootSortKeyBuilder::new()
            .id(&self.user_id)
            .message(MessageSortKey::LastMessageToken)
            .build();

        let attributes = vec!["last_message_token"];

        let res = self.try_generic_query(sort_key, &attributes).await?;

        Ok(res)
    }

    pub async fn try_get_encounters(
        &self,
        game_id: &str,
        level: &str,
        encounter: EncounterSortKey,
    ) -> anyhow::Result<Vec<EncounterQuery>> {
        let sk = format!(
            "{}#Game#Level#{}#Encounter#{}#Round#",
            game_id, level, encounter
        );
        info!("Sk: {}", sk);
        let res = self
            .try_generic_begins_with_query(self.user_id.to_string(), sk, vec!["text", "name"])
            .await
            .context("try_generic_begins_with_query failed")?;

        debug!("Begins with query response: {:?}", res);

        let items = res.items.unwrap();

        let ctxs: Vec<EncounterQuery> =
            serde_dynamo::from_items(items).context("serde_dynamo::from_items failed")?;
        Ok(ctxs)
    }

    pub async fn try_get_settings(&self) -> anyhow::Result<Settings> {
        let game_id = self.try_get_active_game_id().await?;
        let sk = SortKeyFactory::new(&self.user_id)
            .create_game_settings_sk(&game_id)
            .build();

        let res = self.try_generic_get(&sk).await.context(format!(
            "Generic get failed with args;  user_id: {}, sk: {}",
            &self.user_id, &sk,
        ))?;

        let item = res.item.unwrap().clone();

        let state: Settings =
            serde_dynamo::from_item(item).context("serde_dynamo::from_items failed")?;

        Ok(state)
    }

    pub async fn try_get_state(&self) -> anyhow::Result<State> {
        let game_id = self.try_get_active_game_id().await?;
        info!("Game Id: {}", game_id);
        let sk = RootSortKeyBuilder::create_state_sk(&game_id).build();
        info!("GameState Sk: {}", sk);

        let attributes = vec!["round", "level", "curr_encounter", "prev_encounter"];

        let res = self
            .try_generic_query(sk.clone(), &attributes)
            .await
            .context(format!(
                "Generic query failed with args; user_id: {}, sk: {}, attrs: {:?}",
                &self.user_id, sk, attributes
            ))?;

        info!("get state res : {:?}", res);

        let item = res.items.unwrap()[0].clone();
        info!("BING");

        info!("item: {:?}", item);

        let state: State =
            serde_dynamo::from_item(item).context("serde_dynamo::from_items failed")?;

        info!("BANG");

        Ok(state)
    }

    pub async fn try_update_state(
        &self,
        updates: HashMap<String, AttributeValue>,
    ) -> anyhow::Result<UpdateItemOutput> {
        let sk = RootSortKeyBuilder::create_state_sk(&self.user_id).build();

        let res = self.try_generic_update(&self.user_id, &sk, updates).await?;

        Ok(res)
    }

    pub async fn try_get_enemies(
        &self,
        game_id: &str,
        round: u8,
        level: u8,
    ) -> anyhow::Result<Vec<BattleScenarioEnemy>> {
        let sk = SortKeyFactory::new(&self.user_id)
            .create_encounter_sk(game_id, round, level, EncounterSortKey::Battle)
            .build();

        let attributes = vec!["enemies"];

        let res = self
            .try_generic_query(sk.clone(), &attributes)
            .await
            .context(format!(
                "Generic query failed with args; user_id: {}, sk: {}, attrs: {:?}",
                &self.user_id, sk, attributes
            ))?;

        let items = res.items.expect(format!("Could not find {}", sk).as_str());

        info!("ENEMIES: {:?}", items);

        let x: Vec<_> = items
            .first()
            .expect("there should be at least 1 item in the list")
            .get("enemies")
            .expect("no enemies key found")
            .as_l()
            .expect("could not convert enemies key to hashmap")
            .to_owned()
            .into_iter()
            .filter_map(|v| v.as_m().ok().cloned())
            .collect();

        let enemies: Vec<BattleScenarioEnemy> = serde_dynamo::from_items(x)
            .context("serde_dynamo::from_items failed creating, BattleScenarioEnemy")?;

        Ok(enemies)
    }

    pub async fn try_get_llm_model(&self) -> anyhow::Result<String> {
        let game_id = self.try_get_active_game_id().await?;
        let sk = SortKeyFactory::new(&self.user_id)
            .create_game_settings_sk(&game_id)
            .build();

        let attribute = "model";
        let res = self
            .try_generic_query(sk.clone(), &vec![attribute])
            .await
            .context(format!(
                "Generic query failed with args; user_id: {}, sk: {}, attrs: {}",
                &self.user_id, sk, attribute
            ))?;

        let items = res.items.expect(format!("Could not find {}", sk).as_str());

        debug!("QUERY: {:?}", items);

        let item = items
            .first()
            .expect("res.items should have at least one item in the list for try_get_llm_model")
            .get_key_value(attribute)
            .expect(&format!("{} not found", attribute))
            .1
            .as_s()
            .unwrap()
            .to_owned();

        Ok(item)
    }

    pub async fn try_get_active_game_id(&self) -> anyhow::Result<String> {
        let sk = SortKeyFactory::new(&self.user_id)
            .create_user_active_game_sk()
            .build();

        let attributes = vec!["game_id"];
        let res = self
            .try_generic_query(sk.clone(), &attributes)
            .await
            .context(format!(
                "Generic query failed with args; user_id: {}, sk: {}, attrs: {}",
                &self.user_id, sk, "game_id"
            ))?;

        let items = res.items.expect(format!("Could not find {}", sk).as_str());

        debug!("QUERY: {:?}", items);

        let game_id = items
            .first()
            .unwrap()
            .get_key_value("game_id")
            .expect("game_id not found")
            .1
            .as_s()
            .unwrap()
            .to_owned();

        Ok(game_id)
    }

    pub async fn try_new_game(&self, data: NewGameData) -> anyhow::Result<NewGameResponse> {
        let game_id = try_create_sqid(None)?;

        let factory = SortKeyFactory::new(&data.user_id);

        let mut sort_keys: Vec<RootSortKeyBuilder> = Vec::new();

        sort_keys.extend(factory.create_all_entity_inventory_sks(&game_id));
        sort_keys.extend(factory.create_all_entity_actions_sks(&game_id));
        sort_keys.extend(factory.create_all_entity_stats_sks(&game_id));

        self.try_generic_batch_write_root_sks(sort_keys).await?;

        let mut update_items = Vec::new();

        let stats = get_base_stats_by_name(&data.class)?;

        info!("Class Stats {:?}", stats);

        for ability in AbilitiesSortKey::iter() {
            let stat = match ability {
                AbilitiesSortKey::Strength => &stats.strength,
                AbilitiesSortKey::Charisma => &stats.charisma,
                AbilitiesSortKey::Constitution => &stats.constitution,
                AbilitiesSortKey::Dexterity => &stats.dexterity,
                AbilitiesSortKey::Intelligence => &stats.intelligence,
                AbilitiesSortKey::Wisdom => &stats.wisdom,
            };

            let sk = RootSortKeyBuilder::create_abilities_sk(&game_id, ability, Entity::Player);

            update_items.push(UpdateItem {
                sort_key: sk,
                item: ("value".to_string(), stat.to_string()),
            });
        }

        let settings = Settings {
            theme: Some(data.theme),
            model: Some("anthropic.claude-3-5-sonnet-20240620-v1:0".to_string()),
        };

        self.try_generic_batch_update(update_items).await?;
        self.try_save_active_game_id(&game_id).await?;
        self.try_save_new_game_state().await?;
        self.try_save_game_settings(&game_id, settings).await?;

        Ok(NewGameResponse { game_id })
    }

    pub async fn try_generic_batch_write_root_sks(
        &self,
        sort_keys: Vec<RootSortKeyBuilder>,
    ) -> anyhow::Result<()> {
        info!("Batch Write Item Count: {}", sort_keys.len());

        let mut items: Vec<HashMap<String, AttributeValue>> = Vec::new();

        let components: Vec<StateComponent> = sort_keys
            .iter()
            .map(|sk| StateComponent {
                user_id: self.user_id.to_string(),
                state_component: sk.build(),
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

    pub async fn try_generic_batch_update(&self, items: Vec<UpdateItem>) -> anyhow::Result<()> {
        info!("Update Item Count: {}", items.len());
        info!("Update Items: {:?}", items);

        let mut transact_items = Vec::new();

        for item in items {
            info!("Sort keys: {}", item.sort_key.build());

            let update_request = Update::builder()
                .table_name(GAME_STATE_TABLE.to_string())
                .key("UserId", AttributeValue::S(self.user_id.to_string()))
                .key("StateComponent", AttributeValue::S(item.sort_key.build()))
                .set_update_expression(Some("SET #attr = :val".to_string()))
                .set_expression_attribute_names(Some(HashMap::from([(
                    "#attr".to_string(),
                    item.item.0,
                )])))
                .set_expression_attribute_values(Some(HashMap::from([(
                    ":val".to_string(),
                    AttributeValue::S(item.item.1),
                )])))
                .build()?;

            let transact_write_item = TransactWriteItem::builder().update(update_request).build();

            info!("Update Item: {:?}", transact_write_item);

            transact_items.push(transact_write_item);
        }

        match self
            .client
            .transact_write_items()
            .set_transact_items(Some(transact_items))
            .send()
            .await
        {
            Ok(response) => {
                debug!("Transact Write Response: {:?}", response);
                info!("Transact write successful!");
                Ok(())
            }
            Err(e) => Err(anyhow!("Error during transact write: {:?}", e)),
        }
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
