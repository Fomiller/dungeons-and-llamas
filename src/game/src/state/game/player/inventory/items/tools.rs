#[derive(strum::Display)]
pub enum ToolSortKey {
    #[strum(to_string = "Artisan")]
    Artisan,
    #[strum(to_string = "Thieves")]
    Thieves,
    #[strum(to_string = "Instrument")]
    Instrument,
}
