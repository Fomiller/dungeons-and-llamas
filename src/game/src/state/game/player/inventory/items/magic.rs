#[derive(strum::Display)]
pub enum MagicItemSortKey {
    #[strum(to_string = "Potion")]
    Potion,
    #[strum(to_string = "Wondrous")]
    Wondrous,
    #[strum(to_string = "Ring")]
    Ring,
}
