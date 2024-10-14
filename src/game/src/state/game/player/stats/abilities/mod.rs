#[derive(strum::Display)]
pub enum AbilitiesSortKey {
    #[strum(to_string = "Strength")]
    Strength,
    #[strum(to_string = "Charisma")]
    Charisma,
    #[strum(to_string = "Constitution")]
    Constitution,
    #[strum(to_string = "Dexterity")]
    Dexterity,
    #[strum(to_string = "Intelligence")]
    Intelligence,
    #[strum(to_string = "Wisdom")]
    Wisdom,
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::RootSortKey;
    use super::super::super::super::GameSortKey;
    use super::super::super::PlayerSortKey;
    use super::super::StatsSortKey;
    use super::AbilitiesSortKey;

    #[test]
    fn test_abilities_sort_key() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Player#Stats#Abilities#Strength",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Strength,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Abilities#Dexterity",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Dexterity,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Abilities#Constitution",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Constitution,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Abilities#Charisma",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Charisma,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Abilities#Intelligence",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Intelligence,
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Stats#Abilities#Wisdom",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Wisdom,
                    ))),
                ),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}

