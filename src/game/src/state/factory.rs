use crate::{
    client::{
        EquippedStateSortKey, InventorySortKeyBuilder, ItemSortKeyBuilder, WeaponSortKey,
        WeaponSortKeyBuilder,
    },
    state::{
        buildable::SortKeyBuildable,
        builder::RootSortKeyBuilder,
        game::{
            entity::{
                actions::{
                    spells::{SpellSortKey, SpellSortKeyIter},
                    ActionsSortKey, ActionsSortKeyBuilder, ActionsSortKeyIter,
                },
                inventory::items::{
                    armor::{ArmorSortKey, ArmorSortKeyBuilder},
                    books_and_scrolls::{BookAndScrollSortKey, BookAndScrollSortKeyIter},
                    magic::{MagicItemSortKey, MagicItemSortKeyIter},
                    tools::{ToolSortKey, ToolSortKeyIter},
                },
                Entity, EntitySortKeyBuilder,
            },
            GameSortKeyBuilder,
        },
    },
};

use strum::IntoEnumIterator;

pub struct SortKeyFactory {
    pub user_id: String,
}

impl SortKeyFactory {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub fn create_all_entity_inventory_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let mut sks = Vec::new();
        let player = self.create_player_inventory_sks(game_id);
        let enemy = self.create_enemy_inventory_sks(game_id);

        sks.extend(player);
        sks.extend(enemy);

        sks
    }

    pub fn create_player_inventory_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let inventory_keys = self.create_inventory_sks();
        inventory_keys
            .iter()
            .map(|sk| {
                let entity_sk = EntitySortKeyBuilder::new(Entity::Player).inventory(*sk);
                let game_sk = GameSortKeyBuilder::new().entity(entity_sk);
                RootSortKeyBuilder::new().id(&game_id).game(game_sk)
            })
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_player_action_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let inventory_keys = self.create_inventory_sks();
        inventory_keys
            .iter()
            .map(|sk| {
                let entity_sk = EntitySortKeyBuilder::new(Entity::Player).inventory(*sk);
                let game_sk = GameSortKeyBuilder::new().entity(entity_sk);
                RootSortKeyBuilder::new().id(&game_id).game(game_sk)
            })
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_enemy_inventory_sks(&self, game_id: &str) -> Vec<RootSortKeyBuilder> {
        let inventory_keys = self.create_inventory_sks();
        inventory_keys
            .iter()
            .map(|sk| {
                let entity_sk = EntitySortKeyBuilder::new(Entity::Enemy).inventory(*sk);
                let game_sk = GameSortKeyBuilder::new().entity(entity_sk);
                RootSortKeyBuilder::new().id(&game_id).game(game_sk)
            })
            .collect::<Vec<RootSortKeyBuilder>>()
    }

    pub fn create_inventory_sks(&self) -> Vec<InventorySortKeyBuilder> {
        let items = self.create_all_item_buildable();
        self.inventory_from_item_buildable(items)
    }

    pub fn create_actions_sks(&self) -> Vec<ActionsSortKeyBuilder> {
        let actions = self.create_all_actions_buildable();
        self.actions_from_buildable(actions)
    }

    pub fn actions_from_buildable(
        &self,
        actions: Vec<Box<dyn SortKeyBuildable>>,
    ) -> Vec<ActionsSortKeyBuilder> {
        actions
            .into_iter()
            .map(|i| ActionsSortKeyBuilder::from(i))
            .collect::<Vec<ActionsSortKeyBuilder>>()
    }

    pub fn inventory_from_item_buildable(
        &self,
        items: Vec<Box<dyn SortKeyBuildable>>,
    ) -> Vec<InventorySortKeyBuilder> {
        let sort_keys = items
            .into_iter()
            .map(|i| ItemSortKeyBuilder::from(i))
            .collect::<Vec<ItemSortKeyBuilder>>()
            .iter()
            .map(|i| InventorySortKeyBuilder { item: Some(*i) })
            .collect();

        sort_keys
    }

    pub fn create_all_actions_buildable(&self) -> Vec<Box<dyn SortKeyBuildable>> {
        type BoxedSKBuildable = Box<dyn SortKeyBuildable>;

        let actions_iter: ActionsSortKeyIter = ActionsSortKey::iter();

        let spells_iter: SpellSortKeyIter = SpellSortKey::iter();

        let actions_skb_vec: Vec<BoxedSKBuildable> = actions_iter
            .flat_map(|a| match a {
                ActionsSortKey::Spells => spells_iter
                    .clone()
                    .map(|s| Box::new(ActionsSortKeyBuilder::new().spells(s)) as BoxedSKBuildable)
                    .collect(),
                ActionsSortKey::Action => {
                    vec![Box::new(ActionsSortKeyBuilder::new().action()) as BoxedSKBuildable]
                }
                ActionsSortKey::Reaction => {
                    vec![Box::new(ActionsSortKeyBuilder::new().reaction()) as BoxedSKBuildable]
                }
                ActionsSortKey::BonusAction => {
                    vec![Box::new(ActionsSortKeyBuilder::new().bonus_action()) as BoxedSKBuildable]
                }
            })
            .collect();

        actions_skb_vec
    }

    pub fn create_all_item_buildable(&self) -> Vec<Box<dyn SortKeyBuildable>> {
        type BoxedSKBuildable = Box<dyn SortKeyBuildable>;

        let mut sort_keys: Vec<BoxedSKBuildable> = Vec::new();

        let weapons: Vec<WeaponSortKey> = WeaponSortKey::iter().collect();
        let armor: Vec<ArmorSortKey> = ArmorSortKey::iter().collect();

        let magic_items_iter: MagicItemSortKeyIter = MagicItemSortKey::iter();
        let tools_iter: ToolSortKeyIter = ToolSortKey::iter();
        let books_scrolls_iter: BookAndScrollSortKeyIter = BookAndScrollSortKey::iter();
        let equipped_state_iter = EquippedStateSortKey::iter();

        let adventure_gear_skb = Box::new(ItemSortKeyBuilder::new().adventuring_gear());
        let currency_skb = Box::new(ItemSortKeyBuilder::new().currency());
        let consumables_skb = Box::new(ItemSortKeyBuilder::new().consumables());
        let miscellaneous_skb = Box::new(ItemSortKeyBuilder::new().miscellaneous());

        let weapons_skb_vec: Vec<BoxedSKBuildable> = equipped_state_iter
            .clone()
            .flat_map(|e| {
                weapons.iter().map(move |w| {
                    Box::new(
                        ItemSortKeyBuilder::new()
                            .weapons(WeaponSortKeyBuilder::new().weapon(*w).equipped(e)),
                    ) as BoxedSKBuildable
                })
            })
            .collect();

        let armor_skb_vec: Vec<BoxedSKBuildable> = equipped_state_iter
            .clone()
            .flat_map(|e| {
                armor.iter().map(move |a| {
                    Box::new(
                        ItemSortKeyBuilder::new()
                            .armor(ArmorSortKeyBuilder::new().armor(*a).equipped(e)),
                    ) as BoxedSKBuildable
                })
            })
            .collect();

        let magic_items_skb_vec: Vec<BoxedSKBuildable> = magic_items_iter
            .map(|m| Box::new(ItemSortKeyBuilder::new().magical(m)) as BoxedSKBuildable)
            .collect();

        let tools_skb_vec: Vec<BoxedSKBuildable> = tools_iter
            .map(|t| Box::new(ItemSortKeyBuilder::new().tools(t)) as BoxedSKBuildable)
            .collect();

        let books_scrolls_skb_vec: Vec<BoxedSKBuildable> = books_scrolls_iter
            .map(|b| Box::new(ItemSortKeyBuilder::new().books_and_scrolls(b)) as BoxedSKBuildable)
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
