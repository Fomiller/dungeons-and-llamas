#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum ClothingSortKey {
    #[strum(to_string = "Travelers")]
    Travelers,
    #[strum(to_string = "Cloak")]
    Cloak,
    #[strum(to_string = "Jewelry")]
    Jewelry,
}
