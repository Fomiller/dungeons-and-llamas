pub mod spells;

use spells::SpellSortKey;

#[derive(strum::Display, strum::EnumIter)]
pub enum ActionsSortKey {
    #[strum(to_string = "Spells#")]
    Spells,
    #[strum(to_string = "Action")]
    Action,
    #[strum(to_string = "BonusAction")]
    BonusAction,
    #[strum(to_string = "Reaction")]
    Reaction,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionsSortKeyBuilder {
    spells: Option<SpellSortKey>,
    action: Option<bool>,
    bonus_action: Option<bool>,
    reaction: Option<bool>,
}

impl ActionsSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spells(mut self, spells: SpellSortKey) -> Self {
        self.spells = Some(spells);
        self
    }
    pub fn bonus_action(mut self, bonus_action: bool) -> Self {
        self.bonus_action = Some(bonus_action);
        self
    }
    pub fn action(mut self, action: bool) -> Self {
        self.action = Some(action);
        self
    }
    pub fn reaction(mut self, reaction: bool) -> Self {
        self.reaction = Some(reaction);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from("GameState#");
        if let Some(spells) = self.spells {
            result.push_str(&format!("#{}", spells.to_string()));
        } else if let Some(action) = self.action {
            result.push_str(&format!("#{}", action));
        } else if let Some(reaction) = self.reaction {
            result.push_str(&format!("#{}", reaction));
        } else if let Some(bonus_action) = self.bonus_action {
            result.push_str(&format!("#{}", bonus_action));
        }
        result
    }
}

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::RootSortKey;
//     use super::super::super::GameSortKey;
//     use super::super::PlayerSortKey;
//     use super::ActionsSortKey;
//
//     #[test]
//     fn test_actions_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Actions#BonusAction",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::BonusAction)),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Actions#Reaction",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::Reaction)),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
