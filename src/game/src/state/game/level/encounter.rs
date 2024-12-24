use crate::state::buildable::SortKeyBuildable;
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
    battle: Option<bool>,
    shop: Option<bool>,
    rest: Option<bool>,
}

impl SortKeyBuildable for EncounterSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Ecounter#"));
        if let Some(_) = self.battle {
            result.push_str(&format!("{}", EncounterSortKey::Battle.to_string()));
        } else if let Some(_) = self.shop {
            result.push_str(&format!("{}", EncounterSortKey::Shop.to_string()));
        } else if let Some(_) = self.rest {
            result.push_str(&format!("{}", EncounterSortKey::Rest.to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
