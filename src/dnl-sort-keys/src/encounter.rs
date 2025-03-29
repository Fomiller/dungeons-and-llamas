use crate::buildable::SortKeyBuildable;
use std::any::Any;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter, strum::EnumString)]
pub enum EncounterSortKey {
    #[strum(to_string = "Battle")]
    Battle,
    #[strum(to_string = "Shop")]
    Shop,
    #[strum(to_string = "Rest")]
    Rest,
    #[strum(to_string = "NewGame")]
    NewGame,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EncounterSortKeyBuilder {
    round: Option<u8>,
    encounter: Option<EncounterSortKey>,
}

impl EncounterSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn round(mut self, round: u8) -> Self {
        self.round = Some(round);
        self
    }

    pub fn encounter(mut self, encounter: EncounterSortKey) -> Self {
        self.encounter = Some(encounter);
        self
    }
}

impl SortKeyBuildable for EncounterSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Encounter#"));
        if let Some(encounter) = self.encounter {
            result.push_str(&format!("{}#", encounter.to_string()));
        }

        if let Some(round) = self.round {
            result.push_str(&format!("Round#{}", round));
        }

        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
