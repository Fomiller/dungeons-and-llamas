pub mod game;
pub mod message;
pub mod user;

use std::any::Any;

use game::{
    player::inventory::items::{
        books_and_scrolls::BookAndScrollSortKeyIter, magic::MagicItemSortKeyIter,
        tools::ToolSortKeyIter,
    },
    GameSortKeyBuilder,
};
use message::MessageSortKey;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use user::UserSortKey;

use crate::{
    client::{
        EquippedStateSortKey, InventorySortKeyBuilder, ItemSortKeyBuilder, PlayerSortKeyBuilder,
        WeaponSortKey, WeaponSortKeyBuilder,
    },
    state::game::player::{
        inventory::items::{
            armor::{ArmorSortKey, ArmorSortKeyBuilder},
            books_and_scrolls::BookAndScrollSortKey,
            clothing::ClothingSortKey,
            magic::MagicItemSortKey,
            tools::ToolSortKey,
        },
        stats::{
            abilities::AbilitiesSortKey, conditions::ConditionsSortKey,
            core_attributes::CoreAttributesSortKey, saving_throws::SavingThrowsSortKey,
            skills::SkillsSortKey, StatsSortKeyBuilder,
        },
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

impl GameState {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
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

    pub fn create_inventory_sks(&self) -> Vec<InventorySortKeyBuilder> {
        let items = self.create_all_item_buildable();
        self.inventory_from_item_buildable(items)
    }
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum RootSortKey {
    #[strum(to_string = "Game#")]
    Game,
    #[strum(to_string = "User#")]
    User,
    #[strum(to_string = "Messages#")]
    Message,
}

#[derive(Default)]
pub struct RootSortKeyBuilder {
    id: String,
    game: Option<GameSortKeyBuilder>,
    user: Option<UserSortKey>,
    message: Option<MessageSortKey>,
}

pub trait SortKeyBuildable: Any {
    fn build(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl SortKeyBuildable for RootSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("{}#", self.id));

        if let Some(game) = self.game {
            result.push_str(&format!("{}", game.build().to_string()));
        }
        if let Some(user) = self.user {
            result.push_str(&format!("{}", user.to_string()));
        }
        if let Some(message) = self.message {
            result.push_str(&format!("{}", message.to_string()));
        }

        result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RootSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn game(mut self, game: GameSortKeyBuilder) -> Self {
        self.game = Some(game);
        self
    }
    pub fn user(mut self, user: UserSortKey) -> Self {
        self.user = Some(user);
        self
    }
    pub fn message(mut self, message: MessageSortKey) -> Self {
        self.message = Some(message);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn create_player_sk(id: String, player: PlayerSortKeyBuilder) -> RootSortKeyBuilder {
        RootSortKeyBuilder::new()
            .id(id)
            .game(GameSortKeyBuilder::new().player(player))
    }

    pub fn create_inventory_sk(
        id: String,
        inventory: InventorySortKeyBuilder,
    ) -> RootSortKeyBuilder {
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_item_sk(id: String, item: ItemSortKeyBuilder) -> RootSortKeyBuilder {
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_weapon_sk(
        id: String,
        weapon: WeaponSortKey,
        equipped: EquippedStateSortKey,
    ) -> RootSortKeyBuilder {
        let weapons = WeaponSortKeyBuilder::new()
            .weapon(weapon)
            .equipped(equipped);
        let item = ItemSortKeyBuilder::new().weapons(weapons);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_armor_sk(
        id: String,
        armor: ArmorSortKey,
        equipped: EquippedStateSortKey,
    ) -> RootSortKeyBuilder {
        let armor = ArmorSortKeyBuilder::new().armor(armor).equipped(equipped);
        let item = ItemSortKeyBuilder::new().armor(armor);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_books_and_scrolls_sk(
        id: String,
        books_and_scrolls: BookAndScrollSortKey,
    ) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().books_and_scrolls(books_and_scrolls);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_magic_sk(id: String, magic: MagicItemSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().magical(magic);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_tool_sk(id: String, tool: ToolSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().tools(tool);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_clothing_sk(id: String, clothing: ClothingSortKey) -> RootSortKeyBuilder {
        let item = ItemSortKeyBuilder::new().clothing(clothing);
        let inventory = InventorySortKeyBuilder::new().item(item);
        let player = PlayerSortKeyBuilder::new().inventory(inventory);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_skills_sk(id: String, skills: SkillsSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().skills(skills);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_abilities_sk(id: String, abilities: AbilitiesSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().abilities(abilities);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_conditions_sk(id: String, conditions: ConditionsSortKey) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().conditions(conditions);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_core_attributes_sk(
        id: String,
        core_attributes: CoreAttributesSortKey,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().core_attributes(core_attributes);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }

    pub fn create_savings_throw_sk(
        id: String,
        saving_throws: SavingThrowsSortKey,
    ) -> RootSortKeyBuilder {
        let stats = StatsSortKeyBuilder::new().saving_throws(saving_throws);
        let player = PlayerSortKeyBuilder::new().stats(stats);
        let game = GameSortKeyBuilder::new().player(player);
        RootSortKeyBuilder::new().id(id).game(game)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        game::{
            player::{
                inventory::items::equipped::EquippedStateSortKey,
                inventory::items::weapons::{WeaponSortKey, WeaponSortKeyBuilder},
                inventory::items::ItemSortKeyBuilder,
                inventory::InventorySortKeyBuilder,
                stats::abilities::AbilitiesSortKey,
                stats::conditions::ConditionsSortKey,
                stats::core_attributes::CoreAttributesSortKey,
                stats::saving_throws::SavingThrowsSortKey,
                stats::skills::SkillsSortKey,
                stats::StatsSortKeyBuilder,
                PlayerSortKeyBuilder,
            },
            GameSortKeyBuilder,
        },
        GameState, RootSortKeyBuilder, SortKeyBuildable,
    };

    #[test]
    fn test_sort_key_builder_weapons() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let equipped_expected = "12345#Game#Player#Inventory#Item#Weapons#Equipped#Melee";
        let unequipped_expected = "12345#Game#Player#Inventory#Item#Weapons#UnEquipped#Melee";

        let weapon_equipped_sk = WeaponSortKeyBuilder::new()
            .weapon(WeaponSortKey::Melee)
            .equipped(EquippedStateSortKey::Equipped);

        let weapon_unequipped_sk = WeaponSortKeyBuilder::new().weapon(WeaponSortKey::Melee);

        for weapon in vec![
            (weapon_equipped_sk, equipped_expected),
            (weapon_unequipped_sk, unequipped_expected),
        ] {
            let item_sk = ItemSortKeyBuilder::new().weapons(weapon.0);
            let inventory_sk = InventorySortKeyBuilder::new().item(item_sk);
            let player_sk = PlayerSortKeyBuilder::new().inventory(inventory_sk);
            let game_sk = GameSortKeyBuilder::new().player(player_sk);
            let sk = RootSortKeyBuilder::new()
                .id(game_id.to_string())
                .game(game_sk)
                .build();
            assert_eq!(weapon.1, sk)
        }
    }

    #[test]
    fn test_sort_key_builder_stats() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";

        let stats_st_str_expected = "12345#Game#Player#Stats#SavingThrows#Strength";
        let stats_ca_ac_expected = "12345#Game#Player#Stats#CoreAttributes#ArmorClass";
        let stats_abilities_wis_expected = "12345#Game#Player#Stats#Abilities#Wisdom";
        let stats_conditions_debuff_expected = "12345#Game#Player#Stats#Conditions#Debuff";
        let stats_defenses_expected = "12345#Game#Player#Stats#Defenses";
        let stats_skills_arc_expected = "12345#Game#Player#Stats#Skills#Arcana";

        let stats_st_str_sk =
            StatsSortKeyBuilder::new().saving_throws(SavingThrowsSortKey::Strength);

        let stats_ca_ac_sk =
            StatsSortKeyBuilder::new().core_attributes(CoreAttributesSortKey::ArmorClass);

        let stats_abilities_wis_sk = StatsSortKeyBuilder::new().abilities(AbilitiesSortKey::Wisdom);

        let stats_conditions_debuff_sk =
            StatsSortKeyBuilder::new().conditions(ConditionsSortKey::Debuff);

        let stats_defenses_sk = StatsSortKeyBuilder::new().defenses(true);

        let stats_skills_arc_sk = StatsSortKeyBuilder::new().skills(SkillsSortKey::Arcana);

        for stat in vec![
            (stats_st_str_sk, stats_st_str_expected),
            (stats_ca_ac_sk, stats_ca_ac_expected),
            (stats_abilities_wis_sk, stats_abilities_wis_expected),
            (stats_conditions_debuff_sk, stats_conditions_debuff_expected),
            (stats_defenses_sk, stats_defenses_expected),
            (stats_skills_arc_sk, stats_skills_arc_expected),
        ] {
            let player_sk = PlayerSortKeyBuilder::new().stats(stat.0);
            let game_sk = GameSortKeyBuilder::new().player(player_sk);
            let sk = RootSortKeyBuilder::new()
                .id(game_id.to_string())
                .game(game_sk)
                .build();

            assert_eq!(stat.1, sk);
        }
    }

    #[test]
    fn test_create_inventory_sks() {
        let game_state = GameState::new("12345");

        let sort_keys: Vec<String> = game_state
            .create_inventory_sks()
            .iter()
            .map(|sk| sk.build())
            .collect();

        let expected_sort_keys = vec![
            "Inventory#Item#Armor#Equipped#Light",
            "Inventory#Item#Armor#Equipped#Medium",
            "Inventory#Item#Armor#Equipped#Heavy",
            "Inventory#Item#Armor#Equipped#Shield",
            "Inventory#Item#Armor#UnEquipped#Light",
            "Inventory#Item#Armor#UnEquipped#Medium",
            "Inventory#Item#Armor#UnEquipped#Heavy",
            "Inventory#Item#Armor#UnEquipped#Shield",
            "Inventory#Item#BooksAndScrolls#Reading",
            "Inventory#Item#BooksAndScrolls#Spellbook",
            "Inventory#Item#BooksAndScrolls#Scrolls",
            "Inventory#Item#Magic#Potion",
            "Inventory#Item#Magic#Wondrous",
            "Inventory#Item#Magic#Ring",
            "Inventory#Item#Tools#Artisan",
            "Inventory#Item#Tools#Thieves",
            "Inventory#Item#Tools#Instrument",
            "Inventory#Item#Weapons#Equipped#Melee",
            "Inventory#Item#Weapons#Equipped#Ranged",
            "Inventory#Item#Weapons#Equipped#Thrown",
            "Inventory#Item#Weapons#UnEquipped#Melee",
            "Inventory#Item#Weapons#UnEquipped#Ranged",
            "Inventory#Item#Weapons#UnEquipped#Thrown",
            "Inventory#Item#AdventuringGear",
            "Inventory#Item#Consumables",
            "Inventory#Item#Currency",
            "Inventory#Item#Miscellaneous",
        ];

        assert_eq!(expected_sort_keys, sort_keys)
    }
}
