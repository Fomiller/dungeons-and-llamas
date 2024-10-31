pub mod abilities;
pub mod conditions;
pub mod core_attributes;
pub mod saving_throws;
pub mod skills;

use crate::state::buildable::SortKeyBuildable;
use abilities::AbilitiesSortKey;
use conditions::ConditionsSortKey;
use core_attributes::CoreAttributesSortKey;
use saving_throws::SavingThrowsSortKey;
use skills::SkillsSortKey;

use std::any::Any;

#[derive(strum::Display, strum::EnumIter)]
pub enum StatsSortKey {
    #[strum(to_string = "Skills#")]
    Skills,
    #[strum(to_string = "SavingThrows#")]
    SavingThrows,
    #[strum(to_string = "CoreAttributes#")]
    CoreAttributes,
    #[strum(to_string = "Abilities#")]
    Abilities,
    #[strum(to_string = "Conditions#")]
    Conditions,
    #[strum(to_string = "Defenses")]
    Defenses,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatsSortKeyBuilder {
    skills: Option<SkillsSortKey>,
    saving_throws: Option<SavingThrowsSortKey>,
    core_attributes: Option<CoreAttributesSortKey>,
    abilities: Option<AbilitiesSortKey>,
    conditions: Option<ConditionsSortKey>,
    defenses: Option<bool>,
}

impl StatsSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defenses(mut self) -> Self {
        self.defenses = Some(true);
        self
    }
    pub fn skills(mut self, skills: SkillsSortKey) -> Self {
        self.skills = Some(skills);
        self
    }
    pub fn saving_throws(mut self, saving_throws: SavingThrowsSortKey) -> Self {
        self.saving_throws = Some(saving_throws);
        self
    }
    pub fn core_attributes(mut self, core_attributes: CoreAttributesSortKey) -> Self {
        self.core_attributes = Some(core_attributes);
        self
    }
    pub fn abilities(mut self, abilities: AbilitiesSortKey) -> Self {
        self.abilities = Some(abilities);
        self
    }
    pub fn conditions(mut self, conditions: ConditionsSortKey) -> Self {
        self.conditions = Some(conditions);
        self
    }
}

impl SortKeyBuildable for StatsSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from("Stats#");
        if let Some(skills) = self.skills {
            result.push_str(&format!("{}{}", StatsSortKey::Skills, skills));
        } else if let Some(conditions) = self.conditions {
            result.push_str(&format!("{}{}", StatsSortKey::Conditions, conditions));
        } else if let Some(saving_throws) = self.saving_throws {
            result.push_str(&format!("{}{}", StatsSortKey::SavingThrows, saving_throws));
        } else if let Some(core_attributes) = self.core_attributes {
            result.push_str(&format!(
                "{}{}",
                StatsSortKey::CoreAttributes,
                core_attributes
            ));
        } else if let Some(abilities) = self.abilities {
            result.push_str(&format!("{}{}", StatsSortKey::Abilities, abilities));
        } else if let Some(_) = self.defenses {
            result.push_str(&format!("{}", StatsSortKey::Defenses));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for StatsSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(skb) = skb.as_any().downcast_ref::<StatsSortKeyBuilder>() {
            return *skb;
        }
        Self::default()
    }
}
