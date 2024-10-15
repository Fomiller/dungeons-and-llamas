pub mod abilities;
pub mod conditions;
pub mod core_attributes;
pub mod saving_throws;
pub mod skills;

use abilities::AbilitiesSortKey;
use conditions::ConditionsSortKey;
use core_attributes::CoreAttributesSortKey;
use saving_throws::SavingThrowsSortKey;
use skills::SkillsSortKey;

#[derive(strum::Display)]
pub enum StatsSortKey {
    #[strum(to_string = "Skills#{0}")]
    Skills(SkillsSortKey),
    #[strum(to_string = "SavingThrows#{0}")]
    SavingThrows(SavingThrowsSortKey),
    #[strum(to_string = "CoreAttributes#{0}")]
    CoreAttributes(CoreAttributesSortKey),
    #[strum(to_string = "Abilities#{0}")]
    Abilities(AbilitiesSortKey),
    #[strum(to_string = "Conditions#{0}")]
    Conditions(ConditionsSortKey),
    #[strum(to_string = "Defenses")]
    Defenses,
}

#[derive(Default)]
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

    pub fn skills(mut self, skills: SkillsSortKey) -> Self {
        self.skills = Some(skills);
        self
    }
    pub fn defenses(mut self, defenses: bool) -> Self {
        self.defenses = Some(defenses);
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

    pub fn build(self) -> String {
        let mut result = String::from("GameState#");
        if let Some(skills) = self.skills {
            result.push_str(&format!("#{}", skills.to_string()));
        } else if let Some(skills) = self.skills {
            result.push_str(&format!("#{}", skills));
        } else if let Some(conditions) = self.conditions {
            result.push_str(&format!("#{}", conditions));
        } else if let Some(saving_throws) = self.saving_throws {
            result.push_str(&format!("#{}", saving_throws));
        } else if let Some(core_attributes) = self.core_attributes {
            result.push_str(&format!("#{}", core_attributes));
        } else if let Some(abilities) = self.abilities {
            result.push_str(&format!("#{}", abilities));
        } else if let Some(defenses) = self.defenses {
            result.push_str(&format!("#{}", defenses));
        }
        result
    }
}
