#[derive(strum::Display)]
pub enum MagicItemSortKey {
    #[strum(to_string = "Potion")]
    Potion,
    #[strum(to_string = "Wondrous")]
    Wondrous,
    #[strum(to_string = "Ring")]
    Ring,
}
#[cfg(test)]
mod tests {
    use super::super::super::super::super::super::RootSortKey;
    use super::super::super::super::super::{GameSortKey, PlayerSortKey};
    use super::super::super::{InventorySortKey, ItemSortKey};
    use super::MagicItemSortKey;

    #[test]
    fn test_magic_sort_key() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Player#Inventory#Item#Magic#Potion",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Magical(MagicItemSortKey::Potion),
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Inventory#Item#Magic#Wondrous",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Magical(MagicItemSortKey::Wondrous),
                    ))),
                ),
            ),
            (
                "12345#Game#Player#Inventory#Item#Magic#Ring",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
                        ItemSortKey::Magical(MagicItemSortKey::Ring),
                    ))),
                ),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}
