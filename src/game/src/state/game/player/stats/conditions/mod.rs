#[derive(strum::Display)]
pub enum ConditionsSortKey {
    #[strum(to_string = "Buff")]
    Buff,
    #[strum(to_string = "Debuff")]
    Debuff,
}
#[cfg(test)]
mod tests {
    use super::super::super::super::super::RootSortKey;
    use super::super::super::super::GameSortKey;
    use super::super::super::PlayerSortKey;
    use super::super::StatsSortKey;
    use super::ConditionsSortKey;

    #[test]
    fn test_conditions_sort_key() {
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Player#Stats#Conditions#Buff",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Conditions(
                        ConditionsSortKey::Buff,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Conditions#Debuff",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Conditions(
                        ConditionsSortKey::Debuff,
                    ))),
                ),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}
