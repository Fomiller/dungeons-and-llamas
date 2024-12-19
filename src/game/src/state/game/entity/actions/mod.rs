pub mod spells;

use crate::state::buildable::SortKeyBuildable;
use spells::SpellSortKey;

use std::any::Any;

#[derive(strum::Display, strum::EnumIter)]
pub enum ActionsSortKey {
    #[strum(to_string = "Spells#")]
    Spells,
    #[strum(to_string = "Action")]
    Action,
    #[strum(to_string = "BonusAction")]
    BonusAction,
    #[strum(to_string = "Reaction")]
    Reaction,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionsSortKeyBuilder {
    spells: Option<SpellSortKey>,
    action: Option<bool>,
    bonus_action: Option<bool>,
    reaction: Option<bool>,
}

impl ActionsSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spells(mut self, spells: SpellSortKey) -> Self {
        self.spells = Some(spells);
        self
    }

    pub fn bonus_action(mut self) -> Self {
        self.bonus_action = Some(true);
        self
    }

    pub fn action(mut self) -> Self {
        self.action = Some(true);
        self
    }

    pub fn reaction(mut self) -> Self {
        self.reaction = Some(true);
        self
    }
}

impl SortKeyBuildable for ActionsSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from("Actions#");
        if let Some(spells) = self.spells {
            result.push_str(&format!("Spells#{}", spells.to_string()));
        } else if let Some(_) = self.action {
            result.push_str(&format!("{}", ActionsSortKey::Action.to_string()));
        } else if let Some(_) = self.reaction {
            result.push_str(&format!("{}", ActionsSortKey::Reaction.to_string()));
        } else if let Some(_) = self.bonus_action {
            result.push_str(&format!("{}", ActionsSortKey::BonusAction.to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for ActionsSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(skb) = skb.as_any().downcast_ref::<ActionsSortKeyBuilder>() {
            return *skb;
        }
        Self::default()
    }
}
