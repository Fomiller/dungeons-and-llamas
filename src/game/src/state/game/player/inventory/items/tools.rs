#[derive(strum::Display)]
pub enum ToolSortKey {
    #[strum(to_string = "Artisan")]
    Artisan,
    #[strum(to_string = "Thieves")]
    Thieves,
    #[strum(to_string = "Instrument")]
    Instrument,
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::super::RootSortKey;
    use super::super::super::super::super::{GameSortKey, PlayerSortKey};
    use super::super::super::{InventorySortKey, ItemSortKey};
    use super::ToolSortKey;

    #[test]
    fn test_tools_sort_key() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Player#Inventory#Item#Tools#Artisan",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Tools(ToolSortKey::Artisan),
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Inventory#Item#Tools#Thieves",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Tools(ToolSortKey::Thieves),
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Inventory#Item#Tools#Instrument",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Tools(ToolSortKey::Instrument),
                    ))),
                ),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}
