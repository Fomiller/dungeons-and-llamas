#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum ToolSortKey {
    #[strum(to_string = "Artisan")]
    Artisan,
    #[strum(to_string = "Thieves")]
    Thieves,
    #[strum(to_string = "Instrument")]
    Instrument,
}
