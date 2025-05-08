use super::color::Rgb;
use super::connection::Point;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum EncounterType {
    #[strum(to_string = ":crossed_swords:")]
    Monster,
    #[strum(to_string = ":skull:")]
    Boss,
    #[strum(to_string = ":dragon:")]
    Elite,
    #[strum(to_string = ":question:")]
    Event,
    #[strum(to_string = ":coin:")]
    Merchant,
    #[strum(to_string = ":bed:")]
    Rest,
    #[strum(to_string = ":moneybag:")]
    Treasure,
    #[strum(to_string = "N")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub visited: bool,
    pub parent: Option<Point>,
    pub color: Rgb,
    pub location: Point,
    pub id: Uuid,
}
