use crate::state::{buildable::SortKeyBuildable, game::map::encounter};
use std::any::Any;

use super::EncounterSortKey;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum RoundSortKey {
    #[strum(to_string = "Encounter#")]
    Encounter,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RoundSortKeyBuilder {
    round: u8,
    encounter: Option<EncounterSortKey>,
}
impl RoundSortKeyBuilder {
    pub fn new(round: u8) -> Self {
        Self {
            round,
            encounter: None,
        }
    }
    pub fn encounter(mut self, encounter: EncounterSortKey) -> Self {
        self.encounter = Some(encounter);
        self
    }
}

impl SortKeyBuildable for RoundSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Round#{}#", self.round));
        if let Some(encounter) = self.encounter {
            result.push_str(&format!("Encounter#{}", encounter.to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for RoundSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(skb) = skb.as_any().downcast_ref::<RoundSortKeyBuilder>() {
            return *skb;
        }
        Self::default()
    }
}
