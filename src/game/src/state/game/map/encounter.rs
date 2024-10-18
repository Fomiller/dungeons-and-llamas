use super::connection::Point;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncounterType {
    #[strum(to_string = "M")]
    Monster,
    #[strum(to_string = "B")]
    Boss,
    #[strum(to_string = "E")]
    Elite,
    #[strum(to_string = "?")]
    Event,
    #[strum(to_string = "$")]
    Merchant,
    #[strum(to_string = "R")]
    Rest,
    #[strum(to_string = "T")]
    Treasure,
    #[strum(to_string = "N")]
    None,
}

#[derive(Debug, Clone)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub visited: bool,
    pub starting_room: bool,
    pub connected: bool,
    pub symbol: String,
    pub parent: Option<Point>,
}
