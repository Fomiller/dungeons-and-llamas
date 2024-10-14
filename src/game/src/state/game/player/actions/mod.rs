pub mod spells;

use spells::SpellSortKey;

#[derive(strum::Display)]
pub enum ActionsSortKey {
    #[strum(to_string = "Spells#{0}")]
    Spells(SpellSortKey),
    #[strum(to_string = "Action")]
    Action,
    #[strum(to_string = "BonusAction")]
    BonusAction,
    #[strum(to_string = "Reaction")]
    Reaction,
}
#[cfg(test)]
mod tests {
    use super::super::super::super::RootSortKey;
    use super::super::super::GameSortKey;
    use super::super::PlayerSortKey;
    use super::ActionsSortKey;

    #[test]
    fn test_actions_sort_key() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Player#Actions#BonusAction",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::BonusAction)),
                ),
            ),
            (
                "12345#Game#Player#Actions#Reaction",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::Reaction)),
                ),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}
