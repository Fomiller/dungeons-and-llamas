use crate::prelude::*;

use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub struct SortKeyFactory {
    pub user_id: String,
}

// pub type BuilderOpt = for<'a> fn(Entity, &'a str) -> Vec<RootSortKeyBuilder>;
pub type BuilderOpt = Box<dyn Fn(Entity, &str) -> Vec<RootSortKeyBuilder>>;

impl SortKeyFactory {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub fn create_user_active_game_sk(&self) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(&self.user_id)
            .user(UserSortKey::ActiveGameId)
    }

    pub fn create_user_metadata_sk(&self) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(&self.user_id)
            .user(UserSortKey::Metadata)
    }

    pub fn create_game_settings_sk(&self, game_id: &str) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new().id(game_id).settings(true)
    }

    pub fn create_encounter_sk(
        &self,
        game_id: &str,
        round: u8,
        level: u8,
        encounter: EncounterSortKey,
    ) -> RootSortKeyBuilder {
        let encounter_sk = EncounterSortKeyBuilder::new()
            .round(round)
            .encounter(encounter);
        let level_sk = LevelSortKeyBuilder::new(level).encounter(encounter_sk);
        let game_sk = GameSortKeyBuilder::new().level(level_sk);

        RootSortKeyBuilder::new().id(game_id).game(game_sk)
    }

    pub fn create_all_entity_inventory_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let entities = vec![Entity::Player, Entity::Enemy];

        entities
            .iter()
            .flat_map(|e| self.create_entity_sks(*e, EntitySortKey::Inventory, game_id))
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_all_entity_actions_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let entities = vec![Entity::Player, Entity::Enemy];

        entities
            .iter()
            .flat_map(|e| self.create_entity_sks(*e, EntitySortKey::Actions, game_id))
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_all_entity_stats_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let entities = vec![Entity::Player, Entity::Enemy];

        entities
            .iter()
            .flat_map(|e| self.create_entity_sks(*e, EntitySortKey::Stats, game_id))
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_entity_sks(
        &self,
        entity: Entity,
        entity_sk: EntitySortKey,
        game_id: &str,
    ) -> Vec<RootSortKeyBuilder> {
        let entity_skbs: Vec<EntitySortKeyBuilder> = match entity_sk {
            EntitySortKey::Inventory => {
                let sks = self.create_inventory_sks();
                sks.into_iter()
                    .map(|sk| EntitySortKeyBuilder::new(entity).inventory(sk))
                    .collect()
            }
            EntitySortKey::Stats => {
                let sks = self.create_stats_sks();
                sks.into_iter()
                    .map(|sk| EntitySortKeyBuilder::new(entity).stats(sk))
                    .collect()
            }
            EntitySortKey::Actions => {
                let sks = self.create_actions_sks();
                sks.into_iter()
                    .map(|sk| EntitySortKeyBuilder::new(entity).actions(sk))
                    .collect()
            }
            EntitySortKey::Character => vec![EntitySortKeyBuilder::new(entity).character(true)],
        };

        entity_skbs
            .into_iter()
            .map(|skb| {
                let game_sk = GameSortKeyBuilder::new().entity(skb);
                RootSortKeyBuilder::new().id(&game_id).game(game_sk)
            })
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_inventory_sks(&self) -> Vec<InventorySortKeyBuilder> {
        let items = self.create_all_items();
        self.inventory_from_items(items)
    }

    pub fn inventory_from_items(
        &self,
        items: Vec<ItemSortKeyBuilder>,
    ) -> Vec<InventorySortKeyBuilder> {
        let sort_keys = items
            .into_iter()
            .map(|i| InventorySortKeyBuilder { item: Some(i) })
            .collect();

        sort_keys
    }

    pub fn create_stats_sks(&self) -> Vec<StatsSortKeyBuilder> {
        let stats_iter: StatsSortKeyIter = StatsSortKey::iter();

        let skills_iter: SkillsSortKeyIter = SkillsSortKey::iter();
        let savings_throws_iter: SavingThrowsSortKeyIter = SavingThrowsSortKey::iter();
        let core_attributes_iter: CoreAttributesSortKeyIter = CoreAttributesSortKey::iter();
        let abilities_iter: AbilitiesSortKeyIter = AbilitiesSortKey::iter();
        let condtions_iter: ConditionsSortKeyIter = ConditionsSortKey::iter();

        stats_iter
            .flat_map(|s| match s {
                StatsSortKey::Skills => skills_iter
                    .clone()
                    .map(|s| StatsSortKeyBuilder::new().skills(s))
                    .collect(),
                StatsSortKey::SavingThrows => savings_throws_iter
                    .clone()
                    .map(|s| StatsSortKeyBuilder::new().saving_throws(s))
                    .collect(),
                StatsSortKey::CoreAttributes => core_attributes_iter
                    .clone()
                    .map(|s| StatsSortKeyBuilder::new().core_attributes(s))
                    .collect(),
                StatsSortKey::Abilities => abilities_iter
                    .clone()
                    .map(|s| StatsSortKeyBuilder::new().abilities(s))
                    .collect(),
                StatsSortKey::Conditions => condtions_iter
                    .clone()
                    .map(|s| StatsSortKeyBuilder::new().conditions(s))
                    .collect(),
                StatsSortKey::Defenses => {
                    vec![StatsSortKeyBuilder::new().defenses()]
                }
            })
            .collect()
    }

    pub fn create_actions_sks(&self) -> Vec<ActionsSortKeyBuilder> {
        let actions_iter: ActionsSortKeyIter = ActionsSortKey::iter();
        let spells_iter: SpellSortKeyIter = SpellSortKey::iter();

        let actions_skb_vec: Vec<ActionsSortKeyBuilder> = actions_iter
            .flat_map(|a| match a {
                ActionsSortKey::Spells => spells_iter
                    .clone()
                    .map(|s| ActionsSortKeyBuilder::new().spells(s))
                    .collect(),
                ActionsSortKey::Action => {
                    vec![ActionsSortKeyBuilder::new().action()]
                }
                ActionsSortKey::Reaction => {
                    vec![ActionsSortKeyBuilder::new().reaction()]
                }
                ActionsSortKey::BonusAction => {
                    vec![ActionsSortKeyBuilder::new().bonus_action()]
                }
            })
            .collect();

        actions_skb_vec
    }

    pub fn create_all_items(&self) -> Vec<ItemSortKeyBuilder> {
        let mut sort_keys: Vec<ItemSortKeyBuilder> = Vec::new();

        let weapons: Vec<WeaponSortKey> = WeaponSortKey::iter().collect();
        let armor: Vec<ArmorSortKey> = ArmorSortKey::iter().collect();

        let magic_items_iter: MagicItemSortKeyIter = MagicItemSortKey::iter();
        let tools_iter: ToolSortKeyIter = ToolSortKey::iter();
        let books_scrolls_iter: BookAndScrollSortKeyIter = BookAndScrollSortKey::iter();
        let equipped_state_iter = EquippedStateSortKey::iter();

        let adventure_gear_skb = ItemSortKeyBuilder::new().adventuring_gear();
        let currency_skb = ItemSortKeyBuilder::new().currency();
        let consumables_skb = ItemSortKeyBuilder::new().consumables();
        let miscellaneous_skb = ItemSortKeyBuilder::new().miscellaneous();

        let weapons_skb_vec: Vec<ItemSortKeyBuilder> = equipped_state_iter
            .clone()
            .flat_map(|e| {
                weapons.iter().map(move |w| {
                    ItemSortKeyBuilder::new()
                        .weapons(WeaponSortKeyBuilder::new().weapon(*w).equipped(e))
                })
            })
            .collect();

        let armor_skb_vec: Vec<ItemSortKeyBuilder> = equipped_state_iter
            .clone()
            .flat_map(|e| {
                armor.iter().map(move |a| {
                    ItemSortKeyBuilder::new()
                        .armor(ArmorSortKeyBuilder::new().armor(*a).equipped(e))
                })
            })
            .collect();

        let magic_items_skb_vec: Vec<ItemSortKeyBuilder> = magic_items_iter
            .map(|m| ItemSortKeyBuilder::new().magical(m))
            .collect();

        let tools_skb_vec: Vec<ItemSortKeyBuilder> = tools_iter
            .map(|t| ItemSortKeyBuilder::new().tools(t))
            .collect();

        let books_scrolls_skb_vec: Vec<ItemSortKeyBuilder> = books_scrolls_iter
            .map(|b| ItemSortKeyBuilder::new().books_and_scrolls(b))
            .collect();

        sort_keys.extend(armor_skb_vec);
        sort_keys.extend(books_scrolls_skb_vec);
        sort_keys.extend(magic_items_skb_vec);
        sort_keys.extend(tools_skb_vec);
        sort_keys.extend(weapons_skb_vec);

        sort_keys.push(adventure_gear_skb);
        sort_keys.push(consumables_skb);
        sort_keys.push(currency_skb);
        sort_keys.push(miscellaneous_skb);

        sort_keys
    }
}
