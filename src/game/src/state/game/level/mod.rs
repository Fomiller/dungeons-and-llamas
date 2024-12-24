use super::super::buildable::SortKeyBuildable;
use std::any::Any;
pub mod encounter;
pub mod round;
use encounter::*;

#[derive(strum::Display, strum::EnumIter)]
pub enum LevelSortKey {
    #[strum(to_string = "Round#")]
    Round,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LevelSortKeyBuilder {
    level: u8,
    encounter: Option<EncounterSortKeyBuilder>,
}

impl LevelSortKeyBuilder {
    pub fn new(level: u8) -> Self {
        Self {
            level,
            encounter: None,
        }
    }

    pub fn encounter(mut self, encounter: EncounterSortKeyBuilder) -> Self {
        self.encounter = Some(encounter);
        self
    }
}

impl SortKeyBuildable for LevelSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = format!("Level#{}#", self.level);
        if let Some(encounter) = self.encounter {
            result.push_str(&format!("{}", encounter.build().to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for LevelSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(skb) = skb.as_any().downcast_ref::<LevelSortKeyBuilder>() {
            return *skb;
        }
        Self::default()
    }
}
