#[derive(strum::Display, Debug, Clone, Copy, strum::EnumIter)]
pub enum MagicItemSortKey {
    #[strum(to_string = "Potion")]
    Potion,
    #[strum(to_string = "Wondrous")]
    Wondrous,
    #[strum(to_string = "Ring")]
    Ring,
}
