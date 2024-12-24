use crate::state::{buildable::SortKeyBuildable, game::map::encounter};
use std::any::Any;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum EncounterSortKey {
    #[strum(to_string = "Battle")]
    Battle,
    #[strum(to_string = "Shop")]
    Shop,
    #[strum(to_string = "Rest")]
    Rest,
}

#[derive(Debug, Clone, Copy)]
pub struct EncounterSortKeyBuilder {
    round: u8,
    encounter: EncounterSortKey,
}
impl EncounterSortKeyBuilder {
    pub fn new(round: u8, encounter: EncounterSortKey) -> Self {
        Self { round, encounter }
    }
}

impl SortKeyBuildable for EncounterSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Encounter#"));
        result.push_str(&format!("{}#", self.encounter.to_string()));

        result.push_str(&format!("Round#{}", self.round));

        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
