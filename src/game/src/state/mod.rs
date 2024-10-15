pub mod game;
pub mod message;
pub mod user;

use game::GameSortKeyBuilder;
use message::MessageSortKey;
use serde::{Deserialize, Serialize};
use user::UserSortKey;

#[derive(strum::Display)]
pub enum RootSortKey {
    #[strum(to_string = "Game#")]
    Game,
    #[strum(to_string = "User#")]
    User,
    #[strum(to_string = "Messages#")]
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
}

#[derive(Default)]
pub struct SortKeyBuilder {
    id: String,
    game: Option<GameSortKeyBuilder>,
    user: Option<UserSortKey>,
    message: Option<MessageSortKey>,
}

impl SortKeyBuilder {
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

    pub fn build(self) -> String {
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
}

#[cfg(test)]
mod tests {

    use super::game::player::inventory::items::equipped::EquippedStateSortKey;
    use super::game::player::inventory::items::weapons::{WeaponSortKey, WeaponSortKeyBuilder};
    use super::game::player::inventory::items::ItemSortKeyBuilder;
    use super::game::player::inventory::InventorySortKeyBuilder;
    use super::game::player::stats::abilities::AbilitiesSortKey;
    use super::game::player::stats::conditions::ConditionsSortKey;
    use super::game::player::stats::core_attributes::CoreAttributesSortKey;
    use super::game::player::stats::saving_throws::SavingThrowsSortKey;
    use super::game::player::stats::skills::SkillsSortKey;
    use super::game::player::stats::StatsSortKeyBuilder;
    use super::game::player::PlayerSortKeyBuilder;
    use super::game::GameSortKeyBuilder;
    use super::SortKeyBuilder;

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
            let sk = SortKeyBuilder::new()
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
            let sk = SortKeyBuilder::new()
                .id(game_id.to_string())
                .game(game_sk)
                .build();

            assert_eq!(stat.1, sk);
        }
    }
}
