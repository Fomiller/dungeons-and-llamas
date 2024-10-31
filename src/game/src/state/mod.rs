pub mod buildable;
pub mod builder;
pub mod component;
pub mod factory;
pub mod game;
pub mod message;
pub mod sort_key;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
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

#[cfg(test)]
mod tests {
    use crate::client::SortKeyFactory;

    use super::buildable::SortKeyBuildable;
    use super::builder::RootSortKeyBuilder;
    use super::game::{
        entity::{
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
            Entity, EntitySortKeyBuilder,
        },
        GameSortKeyBuilder,
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
            let entity_sk = EntitySortKeyBuilder::new(Entity::Player).inventory(inventory_sk);
            let game_sk = GameSortKeyBuilder::new().entity(entity_sk);
            let sk = RootSortKeyBuilder::new().id(game_id).game(game_sk).build();
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
            let entity_sk = EntitySortKeyBuilder::new(Entity::Player).stats(stat.0);
            let game_sk = GameSortKeyBuilder::new().entity(entity_sk);
            let sk = RootSortKeyBuilder::new().id(game_id).game(game_sk).build();

            assert_eq!(stat.1, sk);
        }
    }

    #[test]
    fn test_create_inventory_sks() {
        let factory = SortKeyFactory::new("12345");

        let sort_keys: Vec<String> = factory
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

    #[test]
    fn test_create_player_inventory_sks() {
        let factory = SortKeyFactory::new("12345");
        let game_id = "abcdef";

        let inventory_sks: Vec<String> = factory
            .create_inventory_sks()
            .iter()
            .map(|sk| sk.build())
            .collect();

        let player_sks: Vec<String> = factory
            .create_player_inventory_sks(game_id)
            .iter()
            .map(|sk| sk.build())
            .collect();

        let expected_sks = inventory_sks
            .iter()
            .map(|sk| format!("{}#Game#Player#{}", game_id, sk))
            .collect::<Vec<String>>();

        assert_eq!(expected_sks, player_sks)
    }

    #[test]
    fn test_create_enemy_inventory_sks() {
        let factory = SortKeyFactory::new("12345");
        let game_id = "abcdef";

        let inventory_sks: Vec<String> = factory
            .create_inventory_sks()
            .iter()
            .map(|sk| sk.build())
            .collect();

        let enemy_sks: Vec<String> = factory
            .create_enemy_inventory_sks(game_id)
            .iter()
            .map(|sk| sk.build())
            .collect();

        let expected_sks = inventory_sks
            .iter()
            .map(|sk| format!("{}#Game#Enemy#{}", game_id, sk))
            .collect::<Vec<String>>();

        assert_eq!(expected_sks, enemy_sks)
    }

    #[test]
    fn test_create_all_entity_inventory_sks() {
        let user_id = "12345";
        let game_id = "abcdef";
        let factory = SortKeyFactory::new(user_id);
        let mut expected_sks: Vec<String> = Vec::new();

        let sort_keys: Vec<String> = factory
            .create_all_entity_inventory_sks(game_id)
            .iter()
            .map(|sk| sk.build())
            .collect();

        let expected_player = factory
            .create_player_inventory_sks(game_id)
            .iter()
            .map(|sk| sk.build())
            .collect::<Vec<String>>();

        let expected_enemy = factory
            .create_enemy_inventory_sks(game_id)
            .iter()
            .map(|sk| sk.build())
            .collect::<Vec<String>>();

        expected_sks.extend(expected_player);
        expected_sks.extend(expected_enemy);

        assert_eq!(expected_sks, sort_keys)
    }
}
