#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum EncounterType {
    Monster,
    Boss,
    Elite,
    Event,
    Merchant,
    Rest,
    Reward,
    Treasure,
}

#[derive(Debug, Clone)]
pub struct Encounter {
    encounter_type: EncounterType,
    visited: bool,
    starting_room: bool,
    connected: bool,
    marker: String,
}
